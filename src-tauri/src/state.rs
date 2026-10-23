use crate::settings::Settings;
use parking_lot::Mutex;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

pub struct SessionHandle {
    pub stop_tx: mpsc::UnboundedSender<()>,
    pub task: JoinHandle<()>,
}

pub struct AppState {
    pub settings: Mutex<Settings>,
    pub session: Mutex<Option<SessionHandle>>,
    pub status: Mutex<String>,
    pub message: Mutex<String>,
    pub detected_language: Mutex<Option<String>>,
}

impl AppState {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings: Mutex::new(settings),
            session: Mutex::new(None),
            status: Mutex::new("idle".to_string()),
            message: Mutex::new("Listo para traducir".to_string()),
            detected_language: Mutex::new(None),
        }
    }

    pub fn is_active(&self) -> bool {
        let status = self.status.lock();
        matches!(status.as_str(), "connecting" | "live" | "reconnecting")
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct StatusPayload {
    pub state: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LanguagePayload {
    pub code: String,
    pub name: Option<String>,
}

pub fn set_status(app: &AppHandle, state: &str, message: &str) {
    if let Some(app_state) = app.try_state::<AppState>() {
        *app_state.status.lock() = state.to_string();
        *app_state.message.lock() = message.to_string();
    }
    let _ = app.emit(
        "translation-status",
        StatusPayload {
            state: state.to_string(),
            message: message.to_string(),
        },
    );
}

pub fn set_detected_language(app: &AppHandle, code: &str) {
    if code.trim().is_empty() {
        return;
    }
    if let Some(app_state) = app.try_state::<AppState>() {
        *app_state.detected_language.lock() = Some(code.to_string());
    }
    let _ = app.emit(
        "translation-language",
        LanguagePayload {
            code: code.to_string(),
            name: None,
        },
    );
}

pub fn clear_session(app: &AppHandle) {
    if let Some(app_state) = app.try_state::<AppState>() {
        let handle = app_state.session.lock().take();
        if let Some(handle) = handle {
            drop(handle);
        }
    }
}
