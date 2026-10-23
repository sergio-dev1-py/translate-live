use super::DeviceInfo;
use tauri::AppHandle;

/// Stub para macOS/Linux mientras se implementan los backends nativos.
/// La reproducción (cpal) ya es multiplataforma; la captura del sistema
/// se añadirá con ScreenCaptureKit y PipeWire/PulseAudio en la siguiente fase.
pub struct SystemCapture;

impl SystemCapture {
    pub fn start(
        _app: AppHandle,
        _device_id: String,
        _gain_db: f32,
        _tx: tokio::sync::mpsc::Sender<Vec<i16>>,
    ) -> Result<Self, String> {
        Err(
            "La captura del audio del sistema aún no está implementada en esta plataforma"
                .to_string(),
        )
    }

    pub fn stop(&self) {}
}

pub fn list_capture_devices() -> Result<Vec<DeviceInfo>, String> {
    Ok(vec![DeviceInfo {
        id: "default".to_string(),
        name: "Sistema (predeterminado)".to_string(),
        is_default: true,
    }])
}
