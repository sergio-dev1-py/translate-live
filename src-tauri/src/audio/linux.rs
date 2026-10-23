use crate::audio::processor::{mono_from_f32, mono_from_i16, mono_from_u16, CaptureProcessor};
use crate::audio::{audio_host, DeviceInfo};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};
use parking_lot::Mutex;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use tauri::AppHandle;

pub struct SystemCapture {
    close_tx: mpsc::Sender<()>,
    handle: Option<JoinHandle<()>>,
}

impl SystemCapture {
    pub fn start(
        app: AppHandle,
        device_id: String,
        gain_db: f32,
        tx: tokio::sync::mpsc::Sender<Vec<i16>>,
    ) -> Result<Self, String> {
        let (close_tx, close_rx) = mpsc::channel::<()>();
        let (ready_tx, ready_rx) = mpsc::sync_channel::<Result<(), String>>(1);

        let handle = thread::Builder::new()
            .name("pulse-capture".into())
            .spawn(move || {
                let host = audio_host();
                let device = match monitor_device(&host, &device_id) {
                    Ok(device) => device,
                    Err(error) => {
                        let _ = ready_tx.send(Err(error));
                        return;
                    }
                };
                let supported = match device.default_input_config() {
                    Ok(config) => config,
                    Err(error) => {
                        let _ = ready_tx.send(Err(format!(
                            "No se pudo leer la configuración del monitor: {error}"
                        )));
                        return;
                    }
                };
                let sample_rate = supported.sample_rate().max(1);
                let channels = supported.channels() as usize;
                let sample_format = supported.sample_format();
                let stream_config: StreamConfig = supported.config();
                let processor = Arc::new(Mutex::new(CaptureProcessor::new(
                    sample_rate,
                    gain_db,
                    tx,
                    app,
                )));

                let stream_result: Result<cpal::Stream, String> = match sample_format {
                    SampleFormat::F32 => {
                        let processor = Arc::clone(&processor);
                        device
                            .build_input_stream(
                                stream_config.clone(),
                                move |data: &[f32], _| {
                                    let mono = mono_from_f32(data, channels);
                                    if let Some(mut processor) = processor.try_lock() {
                                        processor.process(&mono);
                                    }
                                },
                                stream_error,
                                None,
                            )
                            .map_err(|error| format!("No se pudo abrir el monitor: {error}"))
                    }
                    SampleFormat::I16 => {
                        let processor = Arc::clone(&processor);
                        device
                            .build_input_stream(
                                stream_config.clone(),
                                move |data: &[i16], _| {
                                    let mono = mono_from_i16(data, channels);
                                    if let Some(mut processor) = processor.try_lock() {
                                        processor.process(&mono);
                                    }
                                },
                                stream_error,
                                None,
                            )
                            .map_err(|error| format!("No se pudo abrir el monitor: {error}"))
                    }
                    SampleFormat::U16 => {
                        let processor = Arc::clone(&processor);
                        device
                            .build_input_stream(
                                stream_config.clone(),
                                move |data: &[u16], _| {
                                    let mono = mono_from_u16(data, channels);
                                    if let Some(mut processor) = processor.try_lock() {
                                        processor.process(&mono);
                                    }
                                },
                                stream_error,
                                None,
                            )
                            .map_err(|error| format!("No se pudo abrir el monitor: {error}"))
                    }
                    other => Err(format!("Formato de captura no soportado: {other:?}")),
                };

                let stream = match stream_result {
                    Ok(stream) => stream,
                    Err(error) => {
                        let _ = ready_tx.send(Err(error));
                        return;
                    }
                };
                if let Err(error) = stream.play() {
                    let _ = ready_tx.send(Err(format!("No se pudo iniciar el monitor: {error}")));
                    return;
                }

                let _ = ready_tx.send(Ok(()));
                let _ = close_rx.recv();
                drop(stream);
            })
            .map_err(|error| format!("No se pudo crear el hilo de captura: {error}"))?;

        match ready_rx.recv_timeout(Duration::from_secs(8)) {
            Ok(Ok(())) => Ok(Self {
                close_tx,
                handle: Some(handle),
            }),
            Ok(Err(error)) => {
                let _ = close_tx.send(());
                let _ = handle.join();
                Err(error)
            }
            Err(_) => {
                let _ = close_tx.send(());
                let _ = handle.join();
                Err("La captura de PulseAudio no respondió a tiempo".to_string())
            }
        }
    }

    pub fn stop(&self) {
        let _ = self.close_tx.send(());
    }
}

impl Drop for SystemCapture {
    fn drop(&mut self) {
        let _ = self.close_tx.send(());
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn stream_error(error: cpal::Error) {
    eprintln!("[capture] error de entrada: {error}");
}

fn is_monitor(id: &str, name: &str) -> bool {
    id.ends_with(".monitor") || name.to_lowercase().contains("monitor")
}

fn default_sink_id(host: &cpal::Host) -> Option<String> {
    host.default_output_device()
        .and_then(|device| device.id().ok())
        .map(|id| id.id().to_string())
}

fn monitor_device(host: &cpal::Host, device_id: &str) -> Result<cpal::Device, String> {
    let devices = host
        .input_devices()
        .map_err(|error| format!("No se pudieron enumerar las fuentes de audio: {error}"))?;
    let default_sink = default_sink_id(host);
    let mut fallback: Option<cpal::Device> = None;

    for device in devices {
        let id = device
            .id()
            .map(|id| id.id().to_string())
            .unwrap_or_default();
        let name = device
            .description()
            .map(|description| description.name().to_string())
            .unwrap_or_default();
        if !is_monitor(&id, &name) {
            continue;
        }

        if device_id.is_empty() || device_id == "default" {
            if let Some(sink) = &default_sink {
                if id == format!("{sink}.monitor") {
                    return Ok(device);
                }
            }
            if fallback.is_none() {
                fallback = Some(device);
            }
        } else if id == device_id || name == device_id {
            return Ok(device);
        }
    }

    fallback.ok_or_else(|| {
        "No se encontró un monitor de PipeWire/PulseAudio. Activa el monitor del altavoz en tu sistema."
            .to_string()
    })
}

pub fn list_capture_devices() -> Result<Vec<DeviceInfo>, String> {
    let host = audio_host();
    let devices = host
        .input_devices()
        .map_err(|error| format!("No se pudieron enumerar las fuentes de audio: {error}"))?;
    let default_sink = default_sink_id(&host);
    let mut result = Vec::new();

    for device in devices {
        let Ok(id) = device.id() else {
            continue;
        };
        let id = id.id().to_string();
        let Ok(description) = device.description() else {
            continue;
        };
        let name = description.name().to_string();
        if !is_monitor(&id, &name) {
            continue;
        }
        let is_default = default_sink
            .as_ref()
            .map(|sink| id == format!("{sink}.monitor"))
            .unwrap_or(false);
        result.push(DeviceInfo {
            id,
            name,
            is_default,
        });
    }

    Ok(result)
}

pub fn probe_capture(device_id: &str, seconds: u64) -> Result<(f32, f32), String> {
    let host = audio_host();
    let device = monitor_device(&host, device_id)?;
    let supported = device
        .default_input_config()
        .map_err(|error| format!("No se pudo leer el monitor: {error}"))?;
    let channels = supported.channels() as usize;
    let sample_format = supported.sample_format();
    let stream_config: StreamConfig = supported.config();
    let stats = Arc::new(Mutex::new((0.0f64, 0usize, 0.0f32)));

    let stream_result: Result<cpal::Stream, String> = match sample_format {
        SampleFormat::F32 => {
            let stats = Arc::clone(&stats);
            device
                .build_input_stream(
                    stream_config.clone(),
                    move |data: &[f32], _| {
                        let mut stats = stats.lock();
                        for value in mono_from_f32(data, channels) {
                            stats.0 += (value * value) as f64;
                            stats.1 += 1;
                            stats.2 = stats.2.max(value.abs());
                        }
                    },
                    stream_error,
                    None,
                )
                .map_err(|error| error.to_string())
        }
        SampleFormat::I16 => {
            let stats = Arc::clone(&stats);
            device
                .build_input_stream(
                    stream_config.clone(),
                    move |data: &[i16], _| {
                        let mut stats = stats.lock();
                        for value in mono_from_i16(data, channels) {
                            stats.0 += (value * value) as f64;
                            stats.1 += 1;
                            stats.2 = stats.2.max(value.abs());
                        }
                    },
                    stream_error,
                    None,
                )
                .map_err(|error| error.to_string())
        }
        SampleFormat::U16 => {
            let stats = Arc::clone(&stats);
            device
                .build_input_stream(
                    stream_config.clone(),
                    move |data: &[u16], _| {
                        let mut stats = stats.lock();
                        for value in mono_from_u16(data, channels) {
                            stats.0 += (value * value) as f64;
                            stats.1 += 1;
                            stats.2 = stats.2.max(value.abs());
                        }
                    },
                    stream_error,
                    None,
                )
                .map_err(|error| error.to_string())
        }
        other => Err(format!("Formato de captura no soportado: {other:?}")),
    };

    let stream = stream_result?;
    stream
        .play()
        .map_err(|error| format!("No se pudo iniciar la prueba: {error}"))?;
    thread::sleep(Duration::from_secs(seconds.clamp(1, 10)));
    drop(stream);

    let (sum, count, peak) = *stats.lock();
    let rms = (sum / count.max(1) as f64).sqrt() as f32;
    Ok((rms, peak))
}
