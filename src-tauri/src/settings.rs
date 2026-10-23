use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub target_language: String,
    /// "default" o el ID de endpoint de WASAPI.
    pub capture_device_id: String,
    /// Cadena vacía = dispositivo de salida predeterminado.
    pub output_device_name: String,
    pub output_volume: f32,
    pub input_gain_db: f32,
    pub echo_target_language: bool,
    pub auto_reconnect: bool,
    /// Idioma de la interfaz (no el idioma de traducción).
    pub ui_language: String,
    pub theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            target_language: "es".to_string(),
            capture_device_id: "default".to_string(),
            output_device_name: String::new(),
            output_volume: 1.0,
            input_gain_db: 0.0,
            echo_target_language: false,
            auto_reconnect: true,
            ui_language: "es".to_string(),
            theme: "light".to_string(),
        }
    }
}

impl Settings {
    pub fn normalized(mut self) -> Self {
        self.output_volume = self.output_volume.clamp(0.0, 1.0);
        self.input_gain_db = self.input_gain_db.clamp(-12.0, 12.0);
        if self.target_language.trim().is_empty() {
            self.target_language = "es".to_string();
        }
        if self.capture_device_id.trim().is_empty() {
            self.capture_device_id = "default".to_string();
        }
        if self.ui_language.trim().is_empty() {
            self.ui_language = "es".to_string();
        }
        if self.theme != "dark" {
            self.theme = "light".to_string();
        }
        self
    }

    pub fn load() -> Self {
        let path = settings_path();
        if !path.exists() {
            return Self::default();
        }
        fs::read_to_string(&path)
            .ok()
            .and_then(|content| serde_json::from_str::<Self>(&content).ok())
            .unwrap_or_default()
            .normalized()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = settings_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("No se pudo crear la carpeta de configuración: {e}"))?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("No se pudo serializar la configuración: {e}"))?;
        fs::write(&path, json).map_err(|e| format!("No se pudo guardar la configuración: {e}"))?;
        Ok(())
    }
}

pub fn settings_path() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("com.translatelive.app").join("settings.json")
}
