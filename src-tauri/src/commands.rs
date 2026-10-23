use crate::audio::{self, playback::PlaybackHandle, DeviceInfo, SystemCapture};
use crate::gemini::{self, SessionConfig};
use crate::secrets;
use crate::settings::Settings;
use crate::state::{clear_session, set_status, AppState, SessionHandle};
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Manager};
use tauri_plugin_opener::OpenerExt;
use tokio::sync::mpsc;

#[derive(Debug, Serialize)]
pub struct Bootstrap {
    pub settings: Settings,
    pub has_api_key: bool,
    pub active: bool,
    pub state: String,
    pub message: String,
    pub detected_language: Option<String>,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct ProbeResult {
    pub rms: f32,
    pub peak: f32,
    pub seconds: f32,
}

#[tauri::command]
pub fn get_bootstrap(app: AppHandle) -> Result<Bootstrap, String> {
    let state = app.state::<AppState>();
    let settings = state.settings.lock().clone();
    let status = state.status.lock().clone();
    let message = state.message.lock().clone();
    let detected = state.detected_language.lock().clone();
    Ok(Bootstrap {
        settings,
        has_api_key: secrets::has_api_key(),
        active: state.is_active(),
        state: status,
        message,
        detected_language: detected,
        version: app.package_info().version.to_string(),
    })
}

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<Settings, String> {
    let state = app.state::<AppState>();
    let settings = state.settings.lock().clone();
    Ok(settings)
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: Settings) -> Result<(), String> {
    let settings = settings.normalized();
    settings.save()?;
    {
        let state = app.state::<AppState>();
        *state.settings.lock() = settings;
    }
    let _ = tauri::Emitter::emit(&app, "settings-changed", ());
    Ok(())
}

#[tauri::command]
pub fn save_api_key(key: String) -> Result<(), String> {
    let key = key.trim();
    if key.is_empty() {
        return Err("La API key está vacía".to_string());
    }
    secrets::save_api_key(key)
}

#[tauri::command]
pub async fn validate_api_key(app: AppHandle, key: Option<String>) -> Result<String, String> {
    let key = key
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .or_else(|| secrets::load_api_key().ok())
        .ok_or_else(|| "Configura una API key primero".to_string())?;
    let target = app
        .state::<AppState>()
        .settings
        .lock()
        .target_language
        .clone();
    gemini::test_connection(&key, &target).await
}

#[tauri::command]
pub fn list_capture_devices() -> Result<Vec<DeviceInfo>, String> {
    audio::list_capture_devices()
}

#[tauri::command]
pub fn list_output_devices() -> Result<Vec<DeviceInfo>, String> {
    audio::list_output_devices()
}

#[tauri::command]
pub async fn start_translation(app: AppHandle) -> Result<(), String> {
    start_translation_inner(&app).await
}

#[tauri::command]
pub async fn stop_translation(app: AppHandle) -> Result<(), String> {
    stop_translation_inner(&app).await
}

pub async fn start_translation_inner(app: &AppHandle) -> Result<(), String> {
    let state = app.state::<AppState>();

    if state.session.lock().is_some() {
        stop_translation_inner(app).await?;
    }

    let api_key = secrets::load_api_key()
        .map_err(|_| "Configura tu API key de Google en Configuración para empezar".to_string())?;
    if api_key.trim().is_empty() {
        show_settings(app);
        return Err("Configura tu API key de Google en Configuración".to_string());
    }

    let settings = state.settings.lock().clone();

    let playback_device = settings.output_device_name.clone();
    let playback_volume = settings.output_volume;
    let playback = tokio::task::spawn_blocking(move || {
        PlaybackHandle::start(&playback_device, playback_volume)
    })
    .await
    .map_err(|e| format!("Error interno al abrir la salida: {e}"))??;
    let playback = Arc::new(playback);

    let (audio_tx, audio_rx) = mpsc::channel::<Vec<i16>>(32);
    let capture_app = app.clone();
    let capture_device = settings.capture_device_id.clone();
    let gain_db = settings.input_gain_db;
    let capture = match tokio::task::spawn_blocking(move || {
        SystemCapture::start(capture_app, capture_device, gain_db, audio_tx)
    })
    .await
    {
        Ok(Ok(capture)) => capture,
        Ok(Err(error)) => {
            return Err(error);
        }
        Err(error) => {
            return Err(format!("Error interno al abrir la captura: {error}"));
        }
    };

    let (stop_tx, stop_rx) = mpsc::unbounded_channel::<()>();
    let config = SessionConfig {
        api_key,
        target_language: settings.target_language.clone(),
        echo_target_language: settings.echo_target_language,
        auto_reconnect: settings.auto_reconnect,
    };

    let task_app = app.clone();
    let playback_task = Arc::clone(&playback);
    let task = tokio::spawn(async move {
        gemini::run_session(task_app, config, audio_rx, playback_task, capture, stop_rx).await;
    });

    {
        let mut session = state.session.lock();
        if session.is_some() {
            drop(session);
            task.abort();
            return Err("Ya hay una traducción activa".to_string());
        }
        *session = Some(SessionHandle { stop_tx, task });
    }

    set_status(app, "connecting", "Conectando con Gemini…");
    Ok(())
}

pub async fn stop_translation_inner(app: &AppHandle) -> Result<(), String> {
    let state = app.state::<AppState>();
    let handle = state.session.lock().take();
    if let Some(handle) = handle {
        let _ = handle.stop_tx.send(());
        let mut task = handle.task;
        tokio::select! {
            _ = &mut task => {}
            _ = tokio::time::sleep(Duration::from_secs(3)) => {
                task.abort();
            }
        }
    }
    set_status(app, "idle", "Listo para traducir");
    Ok(())
}

pub async fn toggle_translation_inner(app: &AppHandle) -> Result<(), String> {
    let active = {
        let state = app.state::<AppState>();
        state.is_active() || state.session.lock().is_some()
    };
    if active {
        stop_translation_inner(app).await
    } else {
        start_translation_inner(app).await
    }
}

pub fn show_settings(app: &AppHandle) {
    if let Some(pill) = app.get_webview_window("pill") {
        let _ = pill.hide();
    }
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

pub fn hide_settings(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.hide();
    }
    if let Some(pill) = app.get_webview_window("pill") {
        let _ = pill.set_always_on_top(true);
        let _ = pill.show();
    }
}

#[tauri::command]
pub fn open_settings(app: AppHandle) {
    show_settings(&app);
}

#[tauri::command]
pub fn close_settings(app: AppHandle) {
    hide_settings(&app);
}

#[tauri::command]
pub fn save_theme(app: AppHandle, theme: String) -> Result<(), String> {
    let theme = if theme == "dark" { "dark" } else { "light" };
    let state = app.state::<AppState>();
    let mut settings = state.settings.lock().clone();
    settings.theme = theme.to_string();
    settings.save()?;
    *state.settings.lock() = settings;
    let _ = tauri::Emitter::emit(&app, "settings-changed", ());
    Ok(())
}

#[tauri::command]
pub fn save_appearance(app: AppHandle, theme: String, lang: String) -> Result<(), String> {
    let theme = if theme == "dark" { "dark" } else { "light" };
    let lang = if lang.trim().is_empty() {
        "es"
    } else {
        lang.trim()
    };
    let state = app.state::<AppState>();
    let mut settings = state.settings.lock().clone();
    settings.theme = theme.to_string();
    settings.ui_language = lang.to_string();
    settings.save()?;
    *state.settings.lock() = settings;
    let _ = tauri::Emitter::emit(&app, "settings-changed", ());
    Ok(())
}

#[tauri::command]
pub fn restart_app(app: AppHandle) {
    app.restart();
}

#[tauri::command]
pub fn set_tray_labels(app: AppHandle, labels: crate::tray::TrayLabels) -> Result<(), String> {
    crate::tray::apply(&app, &labels).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_window_title(app: AppHandle, title: String) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        window
            .set_title(&title)
            .map_err(|error| format!("No se pudo cambiar el título: {error}"))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn test_output(device: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || crate::audio::playback::play_test_tone(&device, 1.0))
        .await
        .map_err(|e| format!("Error interno al reproducir el tono: {e}"))?
}

#[tauri::command]
pub async fn probe_capture(device: String) -> Result<ProbeResult, String> {
    let (rms, peak) = tokio::task::spawn_blocking(move || audio::probe_capture(&device, 3))
        .await
        .map_err(|e| format!("Error interno al probar la captura: {e}"))??;
    Ok(ProbeResult {
        rms,
        peak,
        seconds: 3.0,
    })
}

#[tauri::command]
pub fn open_url(app: AppHandle, url: String) -> Result<(), String> {
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| format!("No se pudo abrir el navegador: {e}"))
}

#[tauri::command]
pub fn quit(app: AppHandle) {
    clear_session(&app);
    app.exit(0);
}
