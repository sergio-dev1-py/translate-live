use crate::audio::playback::PlaybackHandle;
use crate::audio::SystemCapture;
use crate::state::{clear_session, set_detected_language, set_status};
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::AppHandle;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

const MODEL: &str = "gemini-3.5-live-translate-preview";
const WS_URL: &str =
    "wss://generativelanguage.googleapis.com/ws/google.ai.generativelanguage.v1beta.GenerativeService.BidiGenerateContent";
const SESSION_SECONDS: u64 = 13 * 60;
static AUDIO_CHUNKS: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub struct SessionConfig {
    pub api_key: String,
    pub target_language: String,
    pub echo_target_language: bool,
    pub auto_reconnect: bool,
}

#[derive(Debug)]
enum Outcome {
    Stopped,
    Expired,
}

#[derive(Debug)]
struct SessionError {
    message: String,
    retryable: bool,
}

impl SessionError {
    fn retryable(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            retryable: true,
        }
    }

    fn fatal(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            retryable: false,
        }
    }
}

pub async fn run_session(
    app: AppHandle,
    config: SessionConfig,
    mut audio_rx: mpsc::Receiver<Vec<i16>>,
    playback: Arc<PlaybackHandle>,
    capture: SystemCapture,
    mut stop_rx: mpsc::UnboundedReceiver<()>,
) {
    let mut attempt: u32 = 0;
    let mut stopped = false;

    loop {
        if stopped {
            break;
        }

        if attempt == 0 {
            set_status(&app, "connecting", "Conectando con Gemini…");
        } else {
            set_status(
                &app,
                "reconnecting",
                &format!("Reconectando… intento {attempt}"),
            );
        }

        match run_once(&app, &config, &mut audio_rx, &playback, &mut stop_rx).await {
            Ok(Outcome::Stopped) => {
                stopped = true;
            }
            Ok(Outcome::Expired) => {
                if !config.auto_reconnect {
                    set_status(
                        &app,
                        "error",
                        "Sesión finalizada. Pulsa para volver a empezar.",
                    );
                    break;
                }
                attempt = 0;
                playback.clear();
                set_status(&app, "reconnecting", "Renovando sesión…");
                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_millis(350)) => {}
                    _ = stop_rx.recv() => { stopped = true; }
                }
            }
            Err(error) => {
                playback.clear();
                if !config.auto_reconnect || !error.retryable || attempt >= 5 {
                    set_status(&app, "error", &error.message);
                    break;
                }
                attempt += 1;
                let delay = Duration::from_secs(2u64.pow(attempt.min(4)));
                set_status(
                    &app,
                    "reconnecting",
                    &format!("Sin conexión. Reintentando en {} s…", delay.as_secs()),
                );
                tokio::select! {
                    _ = tokio::time::sleep(delay) => {}
                    _ = stop_rx.recv() => { stopped = true; }
                }
            }
        }
    }

    capture.stop();
    playback.clear();
    if stopped {
        set_status(&app, "idle", "Listo para traducir");
    }
    clear_session(&app);
}

async fn run_once(
    app: &AppHandle,
    config: &SessionConfig,
    audio_rx: &mut mpsc::Receiver<Vec<i16>>,
    playback: &Arc<PlaybackHandle>,
    stop_rx: &mut mpsc::UnboundedReceiver<()>,
) -> Result<Outcome, SessionError> {
    while audio_rx.try_recv().is_ok() {}

    let mut url = Url::parse(WS_URL).map_err(|e| SessionError::fatal(e.to_string()))?;
    url.query_pairs_mut().append_pair("key", &config.api_key);

    let (ws, _) = tokio_tungstenite::connect_async(url.as_str())
        .await
        .map_err(map_connect_error)?;
    let (mut sink, mut stream) = ws.split();

    let setup = json!({
        "setup": {
            "model": format!("models/{MODEL}"),
            "generationConfig": {
                "responseModalities": ["AUDIO"],
                "translationConfig": {
                    "targetLanguageCode": config.target_language,
                    "echoTargetLanguage": config.echo_target_language
                }
            },
            "inputAudioTranscription": {},
            "outputAudioTranscription": {}
        }
    });

    sink.send(Message::Text(setup.to_string().into()))
        .await
        .map_err(|e| SessionError::retryable(format!("No se pudo enviar la configuración: {e}")))?;

    // Esperar setupComplete antes de enviar audio.
    let setup_timer = tokio::time::sleep(Duration::from_secs(15));
    tokio::pin!(setup_timer);
    let mut setup_complete = false;

    while !setup_complete {
        tokio::select! {
            _ = &mut setup_timer => {
                return Err(SessionError::retryable("Gemini no confirmó la sesión a tiempo"));
            }
            _ = stop_rx.recv() => {
                let _ = sink.send(Message::Close(None)).await;
                return Ok(Outcome::Stopped);
            }
            message = stream.next() => {
                match message {
                    Some(Ok(Message::Text(text))) => {
                        let value: Value = serde_json::from_str(&text)
                            .map_err(|e| SessionError::retryable(format!("Respuesta inválida de Gemini: {e}")))?;
                        if value.get("setupComplete").is_some() {
                            setup_complete = true;
                        } else if let Some(error) = value.get("error") {
                            return Err(parse_server_error(error));
                        }
                    }
                    Some(Ok(Message::Binary(bytes))) => {
                        let text = String::from_utf8_lossy(&bytes);
                        let value: Value = serde_json::from_str(&text)
                            .map_err(|e| SessionError::retryable(format!("Respuesta inválida de Gemini: {e}")))?;
                        if value.get("setupComplete").is_some() {
                            setup_complete = true;
                        } else if let Some(error) = value.get("error") {
                            return Err(parse_server_error(error));
                        }
                    }
                    Some(Ok(Message::Close(frame))) => {
                        let reason = frame
                            .map(|f| format!("{}: {}", f.code, f.reason))
                            .unwrap_or_else(|| "cierre remoto".to_string());
                        return Err(SessionError::retryable(format!(
                            "Gemini cerró la conexión ({reason})"
                        )));
                    }
                    Some(Ok(Message::Ping(payload))) => {
                        let _ = sink.send(Message::Pong(payload)).await;
                    }
                    Some(Ok(_)) => {}
                    Some(Err(error)) => {
                        return Err(SessionError::retryable(format!("Error de WebSocket: {error}")));
                    }
                    None => {
                        return Err(SessionError::retryable("La conexión con Gemini terminó"));
                    }
                }
            }
        }
    }

    eprintln!("[gemini] setupComplete; target={}", config.target_language);
    set_status(app, "live", "Traduciendo en tiempo real");

    let session_timer = tokio::time::sleep(session_duration());
    tokio::pin!(session_timer);

    loop {
        tokio::select! {
            biased;
            _ = stop_rx.recv() => {
                let _ = sink.send(Message::Close(None)).await;
                return Ok(Outcome::Stopped);
            }
            _ = &mut session_timer => {
                let _ = sink.send(Message::Close(None)).await;
                return Ok(Outcome::Expired);
            }
            chunk = audio_rx.recv() => {
                if let Some(chunk) = chunk {
                    let bytes = pcm_i16_to_bytes(&chunk);
                    let message = json!({
                        "realtimeInput": {
                            "audio": {
                                "data": B64.encode(bytes),
                                "mimeType": "audio/pcm;rate=16000"
                            }
                        }
                    });
                    if let Err(error) = sink.send(Message::Text(message.to_string().into())).await {
                        return Err(SessionError::retryable(format!("No se pudo enviar audio: {error}")));
                    }
                }
            }
            message = stream.next() => {
                match message {
                    Some(Ok(Message::Text(text))) => {
                        if let Err(error) = handle_server_message(app, playback, text.as_bytes()) {
                            return Err(error);
                        }
                    }
                    Some(Ok(Message::Binary(bytes))) => {
                        if let Err(error) = handle_server_message(app, playback, &bytes) {
                            return Err(error);
                        }
                    }
                    Some(Ok(Message::Close(frame))) => {
                        let reason = frame
                            .map(|f| format!("{}: {}", f.code, f.reason))
                            .unwrap_or_else(|| "cierre remoto".to_string());
                        return Err(SessionError::retryable(format!(
                            "Gemini cerró la conexión ({reason})"
                        )));
                    }
                    Some(Ok(Message::Ping(payload))) => {
                        let _ = sink.send(Message::Pong(payload)).await;
                    }
                    Some(Ok(_)) => {}
                    Some(Err(error)) => {
                        return Err(SessionError::retryable(format!("Error de WebSocket: {error}")));
                    }
                    None => {
                        return Err(SessionError::retryable("La conexión con Gemini terminó"));
                    }
                }
            }
        }
    }
}

fn handle_server_message(
    app: &AppHandle,
    playback: &Arc<PlaybackHandle>,
    bytes: &[u8],
) -> Result<(), SessionError> {
    let text = String::from_utf8_lossy(bytes);
    let value: Value = serde_json::from_str(&text)
        .map_err(|e| SessionError::retryable(format!("Respuesta inválida de Gemini: {e}")))?;

    if let Some(error) = value.get("error") {
        return Err(parse_server_error(error));
    }

    if let Some(go_away) = value.get("goAway") {
        let time_left = go_away
            .get("timeLeft")
            .and_then(|v| v.as_str())
            .unwrap_or("poco tiempo");
        eprintln!("[gemini] GoAway: {time_left}");
        return Err(SessionError::retryable("La sesión será renovada"));
    }

    if let Some(content) = value.get("serverContent") {
        if content
            .get("interrupted")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            playback.clear();
        }

        if let Some(transcription) = content.get("inputTranscription") {
            if let Some(language) = transcription.get("languageCode").and_then(|v| v.as_str()) {
                set_detected_language(app, language);
            }
            if let Some(text) = transcription.get("text").and_then(|v| v.as_str()) {
                if !text.trim().is_empty() {
                    eprintln!("[gemini] input: {text}");
                }
            }
        }

        if let Some(transcription) = content.get("outputTranscription") {
            if let Some(text) = transcription.get("text").and_then(|v| v.as_str()) {
                if !text.trim().is_empty() {
                    eprintln!("[gemini] output: {text}");
                }
            }
        }

        if let Some(parts) = content
            .get("modelTurn")
            .and_then(|turn| turn.get("parts"))
            .and_then(|parts| parts.as_array())
        {
            for part in parts {
                let Some(data) = part
                    .get("inlineData")
                    .and_then(|inline| inline.get("data"))
                    .and_then(|data| data.as_str())
                else {
                    continue;
                };
                if let Ok(bytes) = B64.decode(data) {
                    let samples = pcm_bytes_to_i16(&bytes);
                    if !samples.is_empty() {
                        let count = AUDIO_CHUNKS.fetch_add(1, Ordering::Relaxed) + 1;
                        if count == 1 || count % 25 == 0 {
                            eprintln!(
                                "[gemini] translated audio chunks: {count} ({} samples)",
                                samples.len()
                            );
                        }
                        playback.push_pcm_i16(&samples, crate::audio::OUTPUT_SAMPLE_RATE);
                    }
                }
            }
        }
    }

    Ok(())
}

fn parse_server_error(error: &Value) -> SessionError {
    let code = error.get("code").and_then(|v| v.as_i64()).unwrap_or(0);
    let message = error
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("Error desconocido de Gemini")
        .to_string();
    let retryable = matches!(code, 429 | 500 | 502 | 503 | 504) || code == 0;
    let friendly = match code {
        400 => format!("Solicitud inválida a Gemini: {message}"),
        401 | 403 => "La API key no es válida o no tiene acceso al modelo".to_string(),
        404 => "El modelo no está disponible para esta API key".to_string(),
        429 => "Se alcanzó el límite de uso de Gemini. Inténtalo más tarde".to_string(),
        _ => format!("Gemini: {message}"),
    };
    SessionError {
        message: friendly,
        retryable,
    }
}

fn map_connect_error(error: tokio_tungstenite::tungstenite::Error) -> SessionError {
    use tokio_tungstenite::tungstenite::Error as WsError;
    match error {
        WsError::Http(response) => {
            let status = response.status().as_u16();
            let message = match status {
                401 | 403 => "La API key no es válida o no tiene acceso al modelo".to_string(),
                404 => "El modelo o el endpoint no están disponibles".to_string(),
                429 => "Se alcanzó el límite de uso de Gemini".to_string(),
                _ => format!("Gemini rechazó la conexión (HTTP {status})"),
            };
            SessionError {
                message,
                retryable: matches!(status, 429 | 500 | 502 | 503 | 504),
            }
        }
        other => SessionError::retryable(format!("No se pudo conectar con Gemini: {other}")),
    }
}

fn session_duration() -> Duration {
    let seconds = std::env::var("LIVE_TRANSLATE_SESSION_SECONDS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value >= 5)
        .unwrap_or(SESSION_SECONDS);
    Duration::from_secs(seconds)
}

fn pcm_i16_to_bytes(samples: &[i16]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(samples.len() * 2);
    for sample in samples {
        bytes.extend_from_slice(&sample.to_le_bytes());
    }
    bytes
}

fn pcm_bytes_to_i16(bytes: &[u8]) -> Vec<i16> {
    bytes
        .chunks_exact(2)
        .map(|pair| i16::from_le_bytes([pair[0], pair[1]]))
        .collect()
}

/// Valida la API key y el setup del modelo abriendo una sesión breve.
pub async fn test_connection(api_key: &str, target_language: &str) -> Result<String, String> {
    let mut url = Url::parse(WS_URL).map_err(|e| e.to_string())?;
    url.query_pairs_mut().append_pair("key", api_key);
    let (ws, _) = tokio_tungstenite::connect_async(url.as_str())
        .await
        .map_err(|e| map_connect_error(e).message)?;
    let (mut sink, mut stream) = ws.split();
    let setup = json!({
        "setup": {
            "model": format!("models/{MODEL}"),
            "generationConfig": {
                "responseModalities": ["AUDIO"],
                "translationConfig": {
                    "targetLanguageCode": target_language,
                    "echoTargetLanguage": false
                }
            },
            "inputAudioTranscription": {},
            "outputAudioTranscription": {}
        }
    });
    sink.send(Message::Text(setup.to_string().into()))
        .await
        .map_err(|e| e.to_string())?;

    let deadline = tokio::time::sleep(Duration::from_secs(15));
    tokio::pin!(deadline);
    loop {
        tokio::select! {
            _ = &mut deadline => {
                let _ = sink.send(Message::Close(None)).await;
                return Err("Gemini no confirmó la sesión a tiempo".to_string());
            }
            message = stream.next() => {
                match message {
                    Some(Ok(Message::Text(text))) => {
                        let value: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
                        if value.get("setupComplete").is_some() {
                            let _ = sink.send(Message::Close(None)).await;
                            return Ok("Conexión correcta: Gemini aceptó la API key y el modelo".to_string());
                        }
                        if let Some(error) = value.get("error") {
                            let _ = sink.send(Message::Close(None)).await;
                            return Err(parse_server_error(error).message);
                        }
                    }
                    Some(Ok(Message::Binary(bytes))) => {
                        let text = String::from_utf8_lossy(&bytes);
                        let value: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
                        if value.get("setupComplete").is_some() {
                            let _ = sink.send(Message::Close(None)).await;
                            return Ok("Conexión correcta: Gemini aceptó la API key y el modelo".to_string());
                        }
                        if let Some(error) = value.get("error") {
                            let _ = sink.send(Message::Close(None)).await;
                            return Err(parse_server_error(error).message);
                        }
                    }
                    Some(Ok(Message::Close(frame))) => {
                        let reason = frame
                            .map(|f| format!("{}: {}", f.code, f.reason))
                            .unwrap_or_else(|| "cierre remoto".to_string());
                        return Err(format!("Gemini cerró la conexión ({reason})"));
                    }
                    Some(Ok(Message::Ping(payload))) => {
                        let _ = sink.send(Message::Pong(payload)).await;
                    }
                    Some(Ok(_)) => {}
                    Some(Err(error)) => return Err(error.to_string()),
                    None => return Err("La conexión con Gemini terminó".to_string()),
                }
            }
        }
    }
}
