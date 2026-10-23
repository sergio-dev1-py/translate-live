use crate::audio::resample::StreamingResampler;
use crate::audio::CHUNK_SAMPLES;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};

/// Procesador común para capturas que entregan muestras f32 mono a la tasa
/// nativa del dispositivo: aplica ganancia, resamplea a 16 kHz, corta en
/// bloques de 40 ms y publica el nivel para la píldora.
pub struct CaptureProcessor {
    resampler: StreamingResampler,
    pending: VecDeque<i16>,
    tx: tokio::sync::mpsc::Sender<Vec<i16>>,
    app: AppHandle,
    gain: f32,
    level_sum: f64,
    level_count: usize,
    last_level: Instant,
}

impl CaptureProcessor {
    pub fn new(
        source_rate: u32,
        gain_db: f32,
        tx: tokio::sync::mpsc::Sender<Vec<i16>>,
        app: AppHandle,
    ) -> Self {
        Self {
            resampler: StreamingResampler::new(source_rate, crate::audio::TARGET_SAMPLE_RATE),
            pending: VecDeque::with_capacity(CHUNK_SAMPLES * 4),
            tx,
            app,
            gain: 10f32.powf(gain_db.clamp(-24.0, 24.0) / 20.0),
            level_sum: 0.0,
            level_count: 0,
            last_level: Instant::now(),
        }
    }

    pub fn process(&mut self, mono: &[f32]) {
        if mono.is_empty() {
            return;
        }

        for value in mono {
            let amplified = (*value * self.gain).clamp(-1.0, 1.0);
            self.level_sum += (amplified * amplified) as f64;
            self.level_count += 1;
        }

        let resampled = self.resampler.process(mono);
        for value in resampled {
            self.pending
                .push_back((value.clamp(-1.0, 1.0) * 32767.0) as i16);
        }

        while self.pending.len() >= CHUNK_SAMPLES {
            let chunk: Vec<i16> = self.pending.drain(..CHUNK_SAMPLES).collect();
            let _ = self.tx.try_send(chunk);
        }

        if self.last_level.elapsed() >= Duration::from_millis(100) {
            let rms = (self.level_sum / self.level_count.max(1) as f64).sqrt() as f32;
            let _ = self
                .app
                .emit("translation-level", (rms * 4.0).clamp(0.0, 1.0));
            self.level_sum = 0.0;
            self.level_count = 0;
            self.last_level = Instant::now();
        }
    }
}

pub fn mono_from_f32(data: &[f32], channels: usize) -> Vec<f32> {
    if channels <= 1 {
        return data.to_vec();
    }
    data.chunks_exact(channels)
        .map(|frame| frame.iter().copied().sum::<f32>() / channels as f32)
        .collect()
}

pub fn mono_from_i16(data: &[i16], channels: usize) -> Vec<f32> {
    if channels <= 1 {
        return data.iter().map(|sample| *sample as f32 / 32768.0).collect();
    }
    data.chunks_exact(channels)
        .map(|frame| {
            let sum: f32 = frame.iter().map(|sample| *sample as f32 / 32768.0).sum();
            sum / channels as f32
        })
        .collect()
}

pub fn mono_from_u16(data: &[u16], channels: usize) -> Vec<f32> {
    if channels <= 1 {
        return data
            .iter()
            .map(|sample| (*sample as f32 - 32768.0) / 32768.0)
            .collect();
    }
    data.chunks_exact(channels)
        .map(|frame| {
            let sum: f32 = frame
                .iter()
                .map(|sample| (*sample as f32 - 32768.0) / 32768.0)
                .sum();
            sum / channels as f32
        })
        .collect()
}
