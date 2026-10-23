use serde::Deserialize;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Deserialize)]
pub struct TrayLabels {
    pub show: String,
    pub toggle: String,
    pub settings: String,
    pub quit: String,
    pub tooltip: String,
}

impl Default for TrayLabels {
    fn default() -> Self {
        Self {
            show: "Show pill".to_string(),
            toggle: "Start / Stop".to_string(),
            settings: "Settings…".to_string(),
            quit: "Quit".to_string(),
            tooltip: "LiveTranslate — real-time translation".to_string(),
        }
    }
}

pub fn build_menu(app: &AppHandle, labels: &TrayLabels) -> tauri::Result<Menu<tauri::Wry>> {
    let show = MenuItem::with_id(app, "show", &labels.show, true, None::<&str>)?;
    let toggle = MenuItem::with_id(app, "toggle", &labels.toggle, true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", &labels.settings, true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", &labels.quit, true, None::<&str>)?;
    Menu::with_items(app, &[&show, &toggle, &settings, &quit])
}

pub fn setup(app: &AppHandle) -> tauri::Result<()> {
    let labels = TrayLabels::default();
    let menu = build_menu(app, &labels)?;
    let mut builder = TrayIconBuilder::with_id("main")
        .menu(&menu)
        .tooltip(&labels.tooltip)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("pill") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "toggle" => {
                let app = app.clone();
                tauri::async_runtime::spawn(async move {
                    let _ = crate::commands::toggle_translation_inner(&app).await;
                });
            }
            "settings" => crate::commands::show_settings(app),
            "quit" => {
                app.exit(0);
            }
            _ => {}
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder.build(app)?;
    Ok(())
}

pub fn apply(app: &AppHandle, labels: &TrayLabels) -> tauri::Result<()> {
    let menu = build_menu(app, labels)?;
    if let Some(tray) = app.tray_by_id("main") {
        tray.set_menu(Some(menu))?;
        tray.set_tooltip(Some(&labels.tooltip))?;
    }
    Ok(())
}
