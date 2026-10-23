use crate::audio::resample::StreamingResampler;
use crate::audio::DeviceInfo;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct PlaybackHandle {
    buffer: Arc<Mutex<VecDeque<f32>>>,
    close_tx: mpsc::Sender<()>,
    handle: Option<thread::JoinHandle<()>>,
    sample_rate: u32,
    resampler: Mutex<StreamingResampler>,
}

pub fn list_devices() -> Result<Vec<DeviceInfo>, String> {
    PlaybackHandle::list_devices()
}

impl PlaybackHandle {
    pub fn list_devices() -> Result<Vec<DeviceInfo>, String> {
        let host = crate::audio::audio_host();
        let default_name = host
            .default_output_device()
            .and_then(|device| device.description().ok())
            .map(|description| description.name().to_string());
        let mut devices = Vec::new();
        for device in host
            .output_devices()
            .map_err(|e| format!("No se pudieron enumerar las salidas de audio: {e}"))?
        {
            let Ok(description) = device.description() else {
                continue;
            };
            let name = description.name().to_string();
            let is_default = default_name.as_deref() == Some(name.as_str());
            devices.push(DeviceInfo {
                id: name.clone(),
                name,
                is_default,
            });
        }
        Ok(devices)
    }

    pub fn start(device_name: &str, volume: f32) -> Result<Self, String> {
        let host = crate::audio::audio_host();
        let device = if device_name.trim().is_empty() {
            host.default_output_device()
                .ok_or_else(|| "No hay un dispositivo de salida predeterminado".to_string())?
        } else {
            host.output_devices()
                .map_err(|e| format!("No se pudieron enumerar las salidas: {e}"))?
                .find(|d| {
                    d.description()
                        .map(|description| description.name() == device_name)
                        .unwrap_or(false)
                })
                .ok_or_else(|| {
                    let available = Self::list_devices()
                        .map(|devices| {
                            devices
                                .into_iter()
                                .map(|d| d.name)
                                .collect::<Vec<_>>()
                                .join(", ")
                        })
                        .unwrap_or_default();
                    format!("No se encontró la salida '{device_name}'. Disponibles: {available}")
                })?
        };

        let supported = device
            .default_output_config()
            .map_err(|e| format!("No se pudo leer la configuración de salida: {e}"))?;
        let sample_rate = supported.sample_rate();
        let channels = supported.channels() as usize;
        let sample_format = supported.sample_format();
        let config: cpal::StreamConfig = supported.config();

        let buffer = Arc::new(Mutex::new(VecDeque::<f32>::with_capacity(
            sample_rate as usize * 3,
        )));
        let volume_bits = Arc::new(AtomicU32::new(volume.clamp(0.0, 1.0).to_bits()));
        let buffer_for_thread = Arc::clone(&buffer);
        let volume_for_thread = Arc::clone(&volume_bits);
        let (close_tx, close_rx) = mpsc::channel::<()>();
        let (ready_tx, ready_rx) = mpsc::channel::<Result<u32, String>>();

        let handle = thread::Builder::new()
            .name("translation-playback".into())
            .spawn(move || {
                let stream_result = match sample_format {
                    cpal::SampleFormat::F32 => {
                        let buffer = Arc::clone(&buffer_for_thread);
                        let volume = Arc::clone(&volume_for_thread);
                        device.build_output_stream(
                            config,
                            move |data: &mut [f32], _| {
                                write_output(data, channels, &buffer, &volume, |s| s, || 0.0);
                            },
                            stream_error,
                            None,
                        )
                    }
                    cpal::SampleFormat::I16 => {
                        let buffer = Arc::clone(&buffer_for_thread);
                        let volume = Arc::clone(&volume_for_thread);
                        device.build_output_stream(
                            config,
                            move |data: &mut [i16], _| {
                                write_output(
                                    data,
                                    channels,
                                    &buffer,
                                    &volume,
                                    |s| (s.clamp(-1.0, 1.0) * 32767.0) as i16,
                                    || 0,
                                );
                            },
                            stream_error,
                            None,
                        )
                    }
                    cpal::SampleFormat::U16 => {
                        let buffer = Arc::clone(&buffer_for_thread);
                        let volume = Arc::clone(&volume_for_thread);
                        device.build_output_stream(
                            config,
                            move |data: &mut [u16], _| {
                                write_output(
                                    data,
                                    channels,
                                    &buffer,
                                    &volume,
                                    |s| ((s.clamp(-1.0, 1.0) * 32767.0) + 32768.0) as u16,
                                    || 32768,
                                );
                            },
                            stream_error,
                            None,
                        )
                    }
                    cpal::SampleFormat::I32 => {
                        let buffer = Arc::clone(&buffer_for_thread);
                        let volume = Arc::clone(&volume_for_thread);
                        device.build_output_stream(
                            config,
                            move |data: &mut [i32], _| {
                                write_output(
                                    data,
                                    channels,
                                    &buffer,
                                    &volume,
                                    |s| (s.clamp(-1.0, 1.0) * 2147483647.0) as i32,
                                    || 0,
                                );
                            },
                            stream_error,
                            None,
                        )
                    }
                    cpal::SampleFormat::F64 => {
                        let buffer = Arc::clone(&buffer_for_thread);
                        let volume = Arc::clone(&volume_for_thread);
                        device.build_output_stream(
                            config,
                            move |data: &mut [f64], _| {
                                write_output(
                                    data,
                                    channels,
                                    &buffer,
                                    &volume,
                                    |s| s as f64,
                                    || 0.0,
                                );
                            },
                            stream_error,
                            None,
                        )
                    }
                    other => {
                        let _ = ready_tx
                            .send(Err(format!("Formato de salida no soportado: {other:?}")));
                        return;
                    }
                };

                let stream = match stream_result {
                    Ok(stream) => stream,
                    Err(error) => {
                        let _ = ready_tx.send(Err(format!(
                            "No se pudo abrir el dispositivo de salida: {error}"
                        )));
                        return;
                    }
                };

                if let Err(error) = stream.play() {
                    let _ = ready_tx.send(Err(format!("No se pudo iniciar la salida: {error}")));
                    return;
                }

                let _ = ready_tx.send(Ok(sample_rate));
                let _ = close_rx.recv();
                drop(stream);
            })
            .map_err(|e| format!("No se pudo crear el hilo de salida: {e}"))?;

        match ready_rx.recv_timeout(Duration::from_secs(6)) {
            Ok(Ok(rate)) => Ok(Self {
                buffer,
                close_tx,
                handle: Some(handle),
                sample_rate: rate,
                resampler: Mutex::new(StreamingResampler::new(
                    crate::audio::OUTPUT_SAMPLE_RATE,
                    rate,
                )),
            }),
            Ok(Err(error)) => Err(error),
            Err(_) => Err("La salida de audio no respondió a tiempo".to_string()),
        }
    }

    pub fn push_pcm_i16(&self, samples: &[i16], source_rate: u32) {
        if samples.is_empty() {
            return;
        }
        static FIRST_CHUNK: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(true);
        if FIRST_CHUNK.swap(false, Ordering::Relaxed) {
            eprintln!(
                "[playback] first translated chunk: {} samples @ {source_rate} Hz -> {} Hz",
                samples.len(),
                self.sample_rate
            );
        }
        let input: Vec<f32> = samples.iter().map(|&s| s as f32 / 32768.0).collect();
        let output = if source_rate != self.sample_rate {
            self.resampler.lock().process(&input)
        } else {
            input
        };
        self.push_f32(&output);
    }

    fn push_f32(&self, samples: &[f32]) {
        if samples.is_empty() {
            return;
        }
        let mut buffer = self.buffer.lock();
        let max = self.sample_rate as usize * 2;
        if buffer.len() + samples.len() > max {
            let excess = buffer.len() + samples.len() - max;
            let drop_count = excess.min(buffer.len());
            buffer.drain(0..drop_count);
        }
        buffer.extend(samples.iter().copied());
    }

    pub fn clear(&self) {
        self.buffer.lock().clear();
    }
}

impl Drop for PlaybackHandle {
    fn drop(&mut self) {
        let _ = self.close_tx.send(());
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn stream_error(error: cpal::Error) {
    eprintln!("[playback] error de stream: {error}");
}

fn write_output<T, F, S>(
    data: &mut [T],
    channels: usize,
    buffer: &Arc<Mutex<VecDeque<f32>>>,
    volume: &Arc<AtomicU32>,
    convert: F,
    silence: S,
) where
    T: Copy,
    F: Fn(f32) -> T,
    S: Fn() -> T,
{
    let volume = f32::from_bits(volume.load(Ordering::Relaxed));
    let Some(mut buffer) = buffer.try_lock() else {
        let s = silence();
        data.fill(s);
        return;
    };
    for frame in data.chunks_mut(channels) {
        let sample = buffer.pop_front().unwrap_or(0.0) * volume;
        let converted = convert(sample);
        for slot in frame.iter_mut() {
            *slot = converted;
        }
    }
}

pub fn play_test_tone(device_name: &str, volume: f32) -> Result<(), String> {
    let playback = PlaybackHandle::start(device_name, volume)?;
    let sample_rate = crate::audio::OUTPUT_SAMPLE_RATE;
    let duration = 0.65f32;
    let count = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(count);
    for i in 0..count {
        let t = i as f32 / sample_rate as f32;
        let envelope = if i < 240 {
            i as f32 / 240.0
        } else if i > count.saturating_sub(240) {
            (count - i) as f32 / 240.0
        } else {
            1.0
        };
        let value = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.22 * envelope;
        samples.push((value * 32767.0) as i16);
    }
    playback.push_pcm_i16(&samples, sample_rate);
    thread::sleep(Duration::from_millis(850));
    Ok(())
}
