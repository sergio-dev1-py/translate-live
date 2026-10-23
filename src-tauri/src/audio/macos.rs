use crate::audio::processor::CaptureProcessor;
use crate::audio::DeviceInfo;
use screencapturekit::prelude::*;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use tauri::AppHandle;

const SAMPLE_RATE: u32 = 48000;

pub struct SystemCapture {
    close_tx: mpsc::Sender<()>,
    stream_thread: Option<JoinHandle<()>>,
    processor_thread: Option<JoinHandle<()>>,
}

impl SystemCapture {
    pub fn start(
        app: AppHandle,
        _device_id: String,
        gain_db: f32,
        tx: tokio::sync::mpsc::Sender<Vec<i16>>,
    ) -> Result<Self, String> {
        let content = SCShareableContent::get().map_err(|error| {
            format!(
                "Permiso de Grabación de pantalla requerido. Actívalo en Ajustes del Sistema → Privacidad y seguridad → Grabación de pantalla. ({error})"
            )
        })?;
        let display = content
            .displays()
            .into_iter()
            .next()
            .ok_or_else(|| "No se encontró una pantalla para capturar".to_string())?;
        let filter = SCContentFilter::create()
            .with_display(&display)
            .with_excluding_windows(&[])
            .build();
        let config = SCStreamConfiguration::new()
            .with_width(2)
            .with_height(2)
            .with_captures_audio(true)
            .with_excludes_current_process_audio(true)
            .with_sample_rate(SAMPLE_RATE as i32)
            .with_channel_count(2);

        let (audio_tx, audio_rx) = mpsc::channel::<Vec<f32>>();
        let (close_tx, close_rx) = mpsc::channel::<()>();
        let mut stream = SCStream::new(&filter, &config);
        stream.add_output_handler(AudioHandler { sender: audio_tx }, SCStreamOutputType::Audio);
        stream
            .start_capture()
            .map_err(|error| format!("No se pudo iniciar la captura del sistema: {error}"))?;

        let stream_thread = thread::Builder::new()
            .name("sck-stream".into())
            .spawn(move || {
                let _ = close_rx.recv();
                let _ = stream.stop_capture();
            })
            .map_err(|error| format!("No se pudo crear el hilo de ScreenCaptureKit: {error}"))?;

        let processor_thread = thread::Builder::new()
            .name("sck-audio".into())
            .spawn(move || {
                let mut processor = CaptureProcessor::new(SAMPLE_RATE, gain_db, tx, app);
                while let Ok(mono) = audio_rx.recv() {
                    processor.process(&mono);
                }
            })
            .map_err(|error| format!("No se pudo crear el hilo de audio: {error}"))?;

        Ok(Self {
            close_tx,
            stream_thread: Some(stream_thread),
            processor_thread: Some(processor_thread),
        })
    }

    pub fn stop(&self) {
        let _ = self.close_tx.send(());
    }
}

impl Drop for SystemCapture {
    fn drop(&mut self) {
        let _ = self.close_tx.send(());
        if let Some(handle) = self.stream_thread.take() {
            let _ = handle.join();
        }
        if let Some(handle) = self.processor_thread.take() {
            let _ = handle.join();
        }
    }
}

struct AudioHandler {
    sender: mpsc::Sender<Vec<f32>>,
}

impl SCStreamOutputTrait for AudioHandler {
    fn did_output_sample_buffer(&self, sample: CMSampleBuffer, of_type: SCStreamOutputType) {
        if !matches!(of_type, SCStreamOutputType::Audio) {
            return;
        }
        let Some(buffer_list) = sample.audio_buffer_list() else {
            return;
        };

        let mut channels: Vec<Vec<f32>> = Vec::new();
        for buffer in buffer_list.iter() {
            let data = buffer.data();
            if data.len() < 4 {
                continue;
            }
            let samples =
                unsafe { std::slice::from_raw_parts(data.as_ptr() as *const f32, data.len() / 4) };
            let channel_count = (buffer.number_channels as usize).max(1);
            if channel_count == 1 {
                channels.push(samples.to_vec());
            } else {
                let mut mono = Vec::with_capacity(samples.len() / channel_count);
                for frame in samples.chunks_exact(channel_count) {
                    let sum: f32 = frame.iter().copied().sum();
                    mono.push(sum / channel_count as f32);
                }
                channels.push(mono);
            }
        }

        if channels.is_empty() {
            return;
        }
        let frames = channels
            .iter()
            .map(|channel| channel.len())
            .min()
            .unwrap_or(0);
        if frames == 0 {
            return;
        }
        let mut mono = Vec::with_capacity(frames);
        for index in 0..frames {
            let sum: f32 = channels.iter().map(|channel| channel[index]).sum();
            mono.push(sum / channels.len() as f32);
        }
        let _ = self.sender.send(mono);
    }
}

pub fn list_capture_devices() -> Result<Vec<DeviceInfo>, String> {
    Ok(vec![DeviceInfo {
        id: "system".to_string(),
        name: "System audio".to_string(),
        is_default: true,
    }])
}

pub fn probe_capture(_device_id: &str, seconds: u64) -> Result<(f32, f32), String> {
    let content = SCShareableContent::get().map_err(|error| {
        format!(
            "Permiso de Grabación de pantalla requerido. Actívalo en Ajustes del Sistema → Privacidad y seguridad → Grabación de pantalla. ({error})"
        )
    })?;
    let display = content
        .displays()
        .into_iter()
        .next()
        .ok_or_else(|| "No se encontró una pantalla para capturar".to_string())?;
    let filter = SCContentFilter::create()
        .with_display(&display)
        .with_excluding_windows(&[])
        .build();
    let config = SCStreamConfiguration::new()
        .with_width(2)
        .with_height(2)
        .with_captures_audio(true)
        .with_excludes_current_process_audio(true)
        .with_sample_rate(SAMPLE_RATE as i32)
        .with_channel_count(2);

    let (audio_tx, audio_rx) = mpsc::channel::<Vec<f32>>();
    let mut stream = SCStream::new(&filter, &config);
    stream.add_output_handler(AudioHandler { sender: audio_tx }, SCStreamOutputType::Audio);
    stream
        .start_capture()
        .map_err(|error| format!("No se pudo iniciar la prueba: {error}"))?;

    let deadline = Instant::now() + Duration::from_secs(seconds.clamp(1, 10));
    let mut sum = 0.0f64;
    let mut count = 0usize;
    let mut peak = 0.0f32;
    while Instant::now() < deadline {
        match audio_rx.recv_timeout(Duration::from_millis(250)) {
            Ok(mono) => {
                for value in mono {
                    sum += (value * value) as f64;
                    count += 1;
                    peak = peak.max(value.abs());
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => continue,
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
    let _ = stream.stop_capture();
    let rms = (sum / count.max(1) as f64).sqrt() as f32;
    Ok((rms, peak))
}
