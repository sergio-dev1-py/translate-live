use super::resample::StreamingResampler;
use super::{DeviceInfo, CHUNK_SAMPLES, TARGET_SAMPLE_RATE};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, SyncSender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};
use windows::core::{GUID, PCWSTR};
use windows::Win32::Devices::Properties::DEVPKEY_Device_FriendlyName;
use windows::Win32::Foundation::PROPERTYKEY;
use windows::Win32::Media::Audio::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::Variant::VT_LPWSTR;
use windows::Win32::UI::Shell::PropertiesSystem::IPropertyStore;

const WAVE_FORMAT_IEEE_FLOAT: u16 = 3;
const WAVE_FORMAT_EXTENSIBLE: u16 = 0xFFFE;

const KSDATAFORMAT_SUBTYPE_IEEE_FLOAT: GUID =
    GUID::from_u128(0x00000003_0000_0010_8000_00AA00389B71);

pub struct SystemCapture {
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl SystemCapture {
    pub fn start(
        app: AppHandle,
        device_id: String,
        gain_db: f32,
        tx: tokio::sync::mpsc::Sender<Vec<i16>>,
    ) -> Result<Self, String> {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_thread = Arc::clone(&stop);
        let (ready_tx, ready_rx) = mpsc::sync_channel::<Result<(), String>>(1);
        let ready_for_thread = ready_tx.clone();

        let handle = thread::Builder::new()
            .name("wasapi-capture".into())
            .spawn(move || {
                let app_level = app.clone();
                let result = run_capture(
                    &device_id,
                    gain_db,
                    stop_thread,
                    move |chunk| {
                        let _ = tx.try_send(chunk);
                    },
                    move |level| {
                        let _ = app_level.emit("translation-level", level);
                    },
                    Some(ready_tx),
                );
                if let Err(error) = result {
                    eprintln!("[capture] {error}");
                    let _ = ready_for_thread.send(Err(error));
                }
            })
            .map_err(|e| format!("No se pudo crear el hilo de captura: {e}"))?;

        match ready_rx.recv_timeout(Duration::from_secs(6)) {
            Ok(Ok(())) => Ok(Self {
                stop,
                handle: Some(handle),
            }),
            Ok(Err(error)) => {
                stop.store(true, Ordering::SeqCst);
                let _ = handle.join();
                Err(error)
            }
            Err(_) => {
                stop.store(true, Ordering::SeqCst);
                let _ = handle.join();
                Err("La captura de audio no respondió a tiempo".to_string())
            }
        }
    }

    pub fn stop(&self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

impl Drop for SystemCapture {
    fn drop(&mut self) {
        self.stop();
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

pub fn list_capture_devices() -> Result<Vec<DeviceInfo>, String> {
    unsafe {
        let com_initialized = CoInitializeEx(None, COINIT_MULTITHREADED).is_ok();
        let result = (|| {
            let enumerator: IMMDeviceEnumerator =
                CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)
                    .map_err(|e| format!("No se pudo crear el enumerador de audio: {e}"))?;
            let default_id = enumerator
                .GetDefaultAudioEndpoint(eRender, eConsole)
                .ok()
                .and_then(|device| device.GetId().ok())
                .map(|id| pwstr_to_string(id))
                .unwrap_or_default();
            let collection: IMMDeviceCollection = enumerator
                .EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)
                .map_err(|e| format!("No se pudieron enumerar las salidas: {e}"))?;
            let count = collection
                .GetCount()
                .map_err(|e| format!("No se pudo leer el número de dispositivos: {e}"))?;
            let mut devices = Vec::new();
            for index in 0..count {
                let Ok(device) = collection.Item(index) else {
                    continue;
                };
                let Ok(id) = device.GetId() else {
                    continue;
                };
                let id = pwstr_to_string(id);
                let name = get_device_friendly_name(&device).unwrap_or_else(|| id.clone());
                devices.push(DeviceInfo {
                    is_default: !default_id.is_empty() && id == default_id,
                    id,
                    name,
                });
            }
            Ok(devices)
        })();
        if com_initialized {
            CoUninitialize();
        }
        result
    }
}

pub fn probe_capture(device_id: &str, seconds: u64) -> Result<(f32, f32), String> {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_timer = Arc::clone(&stop);
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(seconds));
        stop_timer.store(true, Ordering::SeqCst);
    });

    let mut sum_sq = 0.0f64;
    let mut count = 0usize;
    let mut peak = 0.0f32;
    run_capture(
        device_id,
        0.0,
        stop,
        |chunk| {
            for &sample in &chunk {
                let value = sample as f32 / 32768.0;
                sum_sq += (value * value) as f64;
                count += 1;
                peak = peak.max(value.abs());
            }
        },
        |_| {},
        None,
    )?;
    let rms = (sum_sq / count.max(1) as f64).sqrt() as f32;
    Ok((rms, peak))
}

fn run_capture(
    device_id: &str,
    gain_db: f32,
    stop: Arc<AtomicBool>,
    mut on_chunk: impl FnMut(Vec<i16>),
    mut on_level: impl FnMut(f32),
    ready: Option<SyncSender<Result<(), String>>>,
) -> Result<(), String> {
    unsafe {
        let com_initialized = CoInitializeEx(None, COINIT_MULTITHREADED).is_ok();

        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)
                .map_err(|e| format!("No se pudo crear el enumerador de audio: {e}"))?;

        let device = if device_id.trim().is_empty() || device_id == "default" {
            enumerator
                .GetDefaultAudioEndpoint(eRender, eConsole)
                .map_err(|e| format!("No se pudo abrir la salida predeterminada: {e}"))?
        } else {
            let wide: Vec<u16> = device_id.encode_utf16().chain(std::iter::once(0)).collect();
            enumerator
                .GetDevice(PCWSTR::from_raw(wide.as_ptr()))
                .map_err(|e| format!("No se pudo abrir el dispositivo de captura: {e}"))?
        };

        let device_name = get_device_friendly_name(&device).unwrap_or_else(|| "desconocido".into());
        let audio_client: IAudioClient = device
            .Activate(CLSCTX_ALL, None)
            .map_err(|e| format!("No se pudo activar WASAPI: {e}"))?;
        let format_ptr = audio_client
            .GetMixFormat()
            .map_err(|e| format!("No se pudo leer el formato de mezcla: {e}"))?;
        let format = &*format_ptr;

        let sample_rate = format.nSamplesPerSec;
        let channels = format.nChannels as usize;
        let bits = format.wBitsPerSample;
        let bytes_per_frame = format.nBlockAlign as usize;
        let is_float = is_format_float(format.wFormatTag, format);
        let format_tag = format.wFormatTag;

        audio_client
            .Initialize(
                AUDCLNT_SHAREMODE_SHARED,
                AUDCLNT_STREAMFLAGS_LOOPBACK,
                10_000_000,
                0,
                format_ptr,
                None,
            )
            .map_err(|e| format!("No se pudo iniciar WASAPI loopback: {e}"))?;

        let capture_client: IAudioCaptureClient = audio_client
            .GetService::<IAudioCaptureClient>()
            .map_err(|e| format!("No se pudo obtener el cliente de captura: {e}"))?;

        audio_client
            .Start()
            .map_err(|e| format!("No se pudo iniciar la captura: {e}"))?;

        eprintln!(
            "[capture] {device_name}: {sample_rate} Hz, {channels} ch, {bits} bits, tag={format_tag}, float={is_float}"
        );

        if let Some(ready) = ready {
            let _ = ready.send(Ok(()));
        }

        let mut resampler = StreamingResampler::new(sample_rate, TARGET_SAMPLE_RATE);
        let mut pending: VecDeque<i16> = VecDeque::with_capacity(CHUNK_SAMPLES * 4);
        let gain = 10f32.powf(gain_db.clamp(-24.0, 24.0) / 20.0);
        let mut level_sum = 0.0f64;
        let mut level_count = 0usize;
        let mut last_level = Instant::now();

        while !stop.load(Ordering::SeqCst) {
            let packet_size = match capture_client.GetNextPacketSize() {
                Ok(size) => size,
                Err(_) => break,
            };

            if packet_size == 0 {
                thread::sleep(Duration::from_millis(5));
                continue;
            }

            let mut buffer_ptr: *mut u8 = std::ptr::null_mut();
            let mut frames = 0u32;
            let mut flags = 0u32;

            if capture_client
                .GetBuffer(&mut buffer_ptr, &mut frames, &mut flags, None, None)
                .is_err()
            {
                break;
            }

            if frames > 0 && !buffer_ptr.is_null() {
                let silent = (flags & AUDCLNT_BUFFERFLAGS_SILENT.0 as u32) != 0;
                let mono = if silent {
                    vec![0.0f32; frames as usize]
                } else {
                    buffer_to_mono(
                        buffer_ptr,
                        frames as usize,
                        channels,
                        bits,
                        bytes_per_frame,
                        is_float,
                    )
                };

                for value in &mono {
                    let amplified = (*value * gain).clamp(-1.0, 1.0);
                    level_sum += (amplified * amplified) as f64;
                    level_count += 1;
                }

                let resampled = resampler.process(&mono);
                for value in resampled {
                    let sample = (value.clamp(-1.0, 1.0) * 32767.0) as i16;
                    pending.push_back(sample);
                }

                while pending.len() >= CHUNK_SAMPLES {
                    let chunk: Vec<i16> = pending.drain(..CHUNK_SAMPLES).collect();
                    on_chunk(chunk);
                }

                if last_level.elapsed() >= Duration::from_millis(100) {
                    let rms = (level_sum / level_count.max(1) as f64).sqrt() as f32;
                    on_level((rms * 4.0).clamp(0.0, 1.0));
                    level_sum = 0.0;
                    level_count = 0;
                    last_level = Instant::now();
                }
            }

            let _ = capture_client.ReleaseBuffer(frames);
            thread::sleep(Duration::from_millis(2));
        }

        let _ = audio_client.Stop();
        drop(capture_client);
        drop(audio_client);
        drop(device);
        drop(enumerator);
        CoTaskMemFree(Some(format_ptr as *mut _));
        if com_initialized {
            CoUninitialize();
        }
        on_level(0.0);
        Ok(())
    }
}

fn is_format_float(tag: u16, format: &WAVEFORMATEX) -> bool {
    match tag {
        WAVE_FORMAT_IEEE_FLOAT => true,
        WAVE_FORMAT_EXTENSIBLE => unsafe {
            let ext = format as *const WAVEFORMATEX as *const WAVEFORMATEXTENSIBLE;
            let subformat = std::ptr::addr_of!((*ext).SubFormat).read_unaligned();
            subformat == KSDATAFORMAT_SUBTYPE_IEEE_FLOAT
        },
        _ => false,
    }
}

unsafe fn buffer_to_mono(
    ptr: *const u8,
    frames: usize,
    channels: usize,
    bits: u16,
    bytes_per_frame: usize,
    is_float: bool,
) -> Vec<f32> {
    if channels == 0 || frames == 0 || bytes_per_frame == 0 {
        return Vec::new();
    }
    let data_len = frames.saturating_mul(bytes_per_frame);
    let data = std::slice::from_raw_parts(ptr, data_len);
    let mut mono = Vec::with_capacity(frames);

    if is_float {
        match bits {
            32 => {
                let samples = std::slice::from_raw_parts(ptr as *const f32, data_len / 4);
                for frame in 0..frames {
                    let mut sum = 0.0f64;
                    for channel in 0..channels {
                        let index = frame * channels + channel;
                        if let Some(value) = samples.get(index) {
                            sum += *value as f64;
                        }
                    }
                    mono.push((sum / channels as f64).clamp(-1.0, 1.0) as f32);
                }
            }
            64 => {
                let samples = std::slice::from_raw_parts(ptr as *const f64, data_len / 8);
                for frame in 0..frames {
                    let mut sum = 0.0f64;
                    for channel in 0..channels {
                        let index = frame * channels + channel;
                        if let Some(value) = samples.get(index) {
                            sum += *value;
                        }
                    }
                    mono.push((sum / channels as f64).clamp(-1.0, 1.0) as f32);
                }
            }
            _ => return Vec::new(),
        }
        return mono;
    }

    match bits {
        8 => {
            for frame in 0..frames {
                let mut sum = 0.0f64;
                for channel in 0..channels {
                    let index = frame * channels + channel;
                    if let Some(value) = data.get(index) {
                        sum += (*value as f64 - 128.0) / 128.0;
                    }
                }
                mono.push((sum / channels as f64).clamp(-1.0, 1.0) as f32);
            }
        }
        16 => {
            let samples = std::slice::from_raw_parts(ptr as *const i16, data_len / 2);
            for frame in 0..frames {
                let mut sum = 0i64;
                for channel in 0..channels {
                    let index = frame * channels + channel;
                    if let Some(value) = samples.get(index) {
                        sum += *value as i64;
                    }
                }
                mono.push((sum as f64 / (channels as f64 * 32768.0)).clamp(-1.0, 1.0) as f32);
            }
        }
        24 => {
            for frame in 0..frames {
                let mut sum = 0i64;
                for channel in 0..channels {
                    let index = (frame * channels + channel) * 3;
                    if index + 2 < data.len() {
                        let raw =
                            i32::from_le_bytes([data[index], data[index + 1], data[index + 2], 0]);
                        let sample = (raw << 8) >> 8;
                        sum += sample as i64;
                    }
                }
                mono.push((sum as f64 / (channels as f64 * 8388608.0)).clamp(-1.0, 1.0) as f32);
            }
        }
        32 => {
            let samples = std::slice::from_raw_parts(ptr as *const i32, data_len / 4);
            for frame in 0..frames {
                let mut sum = 0i64;
                for channel in 0..channels {
                    let index = frame * channels + channel;
                    if let Some(value) = samples.get(index) {
                        sum += *value as i64;
                    }
                }
                mono.push((sum as f64 / (channels as f64 * 2147483648.0)).clamp(-1.0, 1.0) as f32);
            }
        }
        _ => return Vec::new(),
    }
    mono
}

unsafe fn pwstr_to_string(pw: windows::core::PWSTR) -> String {
    if pw.is_null() {
        return String::new();
    }
    let mut len = 0usize;
    while *pw.0.add(len) != 0 {
        len += 1;
    }
    let slice = std::slice::from_raw_parts(pw.0 as *const u16, len);
    String::from_utf16_lossy(slice)
}

fn get_device_friendly_name(device: &IMMDevice) -> Option<String> {
    unsafe {
        let store: IPropertyStore = device.OpenPropertyStore(STGM_READ).ok()?;
        let value = store
            .GetValue(&DEVPKEY_Device_FriendlyName as *const _ as *const PROPERTYKEY)
            .ok()?;
        let inner = &value.Anonymous.Anonymous;
        if inner.vt != VT_LPWSTR {
            return None;
        }
        let ptr = inner.Anonymous.pwszVal;
        if ptr.is_null() {
            return None;
        }
        Some(pwstr_to_string(ptr))
    }
}
