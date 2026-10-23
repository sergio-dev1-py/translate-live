use serde::Serialize;

pub mod playback;
pub mod processor;
pub mod resample;

#[cfg(target_os = "windows")]
pub mod wasapi;

#[cfg(target_os = "windows")]
pub use wasapi::SystemCapture;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "macos")]
pub use macos::SystemCapture;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "linux")]
pub use linux::SystemCapture;

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub mod stub;

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub use stub::SystemCapture;

pub const TARGET_SAMPLE_RATE: u32 = 16000;
pub const OUTPUT_SAMPLE_RATE: u32 = 24000;
pub const CHUNK_SAMPLES: usize = 640; // 40 ms a 16 kHz

#[derive(Debug, Clone, Serialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

pub fn list_capture_devices() -> Result<Vec<DeviceInfo>, String> {
    #[cfg(target_os = "windows")]
    {
        return wasapi::list_capture_devices();
    }
    #[cfg(target_os = "macos")]
    {
        return macos::list_capture_devices();
    }
    #[cfg(target_os = "linux")]
    {
        return linux::list_capture_devices();
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        stub::list_capture_devices()
    }
}

/// Host de cpal adecuado para cada plataforma. En Linux usamos el backend
/// nativo de PulseAudio/PipeWire para que la enumeración de monitores y la
/// reproducción funcionen con el servidor de audio del escritorio.
#[cfg(target_os = "linux")]
pub fn audio_host() -> cpal::Host {
    cpal::host_from_id(cpal::HostId::PulseAudio).unwrap_or_else(|_| cpal::default_host())
}

#[cfg(not(target_os = "linux"))]
pub fn audio_host() -> cpal::Host {
    cpal::default_host()
}

pub fn list_output_devices() -> Result<Vec<DeviceInfo>, String> {
    playback::list_devices()
}

#[cfg(target_os = "windows")]
pub fn probe_capture(device_id: &str, seconds: u64) -> Result<(f32, f32), String> {
    wasapi::probe_capture(device_id, seconds)
}

#[cfg(target_os = "macos")]
pub fn probe_capture(device_id: &str, seconds: u64) -> Result<(f32, f32), String> {
    macos::probe_capture(device_id, seconds)
}

#[cfg(target_os = "linux")]
pub fn probe_capture(device_id: &str, seconds: u64) -> Result<(f32, f32), String> {
    linux::probe_capture(device_id, seconds)
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub fn probe_capture(_device_id: &str, _seconds: u64) -> Result<(f32, f32), String> {
    Err("La captura del sistema no está disponible en esta plataforma".to_string())
}
