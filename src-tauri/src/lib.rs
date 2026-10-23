mod audio;
mod commands;
mod gemini;
mod secrets;
mod settings;
mod state;
mod tray;

use settings::Settings;
use state::AppState;
use tauri::Manager;

pub fn run() {
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .try_init();

    let initial_settings = Settings::load();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(window) = app.get_webview_window("pill") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new(initial_settings))
        .setup(|app| {
            position_pill(app.handle());
            tray::setup(app.handle())?;
            setup_shortcut(app.handle())?;
            setup_settings_window(app.handle());

            if !secrets::has_api_key() {
                commands::show_settings(app.handle());
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_bootstrap,
            commands::get_settings,
            commands::save_settings,
            commands::save_api_key,
            commands::validate_api_key,
            commands::list_capture_devices,
            commands::list_output_devices,
            commands::start_translation,
            commands::stop_translation,
            commands::open_settings,
            commands::close_settings,
            commands::save_theme,
            commands::save_appearance,
            commands::restart_app,
            commands::set_tray_labels,
            commands::set_window_title,
            commands::test_output,
            commands::probe_capture,
            commands::open_url,
            commands::quit,
        ])
        .run(tauri::generate_context!())
        .expect("error while running LiveTranslate");
}

fn position_pill(app: &tauri::AppHandle) {
    let Some(window) = app.get_webview_window("pill") else {
        return;
    };
    let monitor = window
        .current_monitor()
        .ok()
        .flatten()
        .or_else(|| window.primary_monitor().ok().flatten());
    let Some(monitor) = monitor else {
        return;
    };
    let size = window
        .outer_size()
        .unwrap_or(tauri::PhysicalSize::new(240, 52));
    let x = monitor.position().x + (monitor.size().width as i32 - size.width as i32) / 2;
    let y = monitor.position().y + monitor.size().height as i32 - size.height as i32 - 64;
    let _ = window.set_position(tauri::PhysicalPosition::new(x, y));
}

fn setup_settings_window(app: &tauri::AppHandle) {
    let Some(settings) = app.get_webview_window("settings") else {
        return;
    };
    let handle = app.clone();
    settings.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            commands::hide_settings(&handle);
        }
    });
}

fn setup_shortcut(app: &tauri::AppHandle) -> tauri::Result<()> {
    use tauri_plugin_global_shortcut::{Builder, ShortcutState};

    let builder = Builder::new();
    let builder = match builder.with_shortcuts(["ctrl+alt+t"]) {
        Ok(builder) => builder,
        Err(error) => {
            eprintln!("[shortcut] no se pudo registrar Ctrl+Alt+T: {error}");
            Builder::new()
        }
    };

    app.plugin(
        builder
            .with_handler(|app, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    let app = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = commands::toggle_translation_inner(&app).await;
                    });
                }
            })
            .build(),
    )?;
    Ok(())
}

pub fn cli_test_gemini() -> Result<(), String> {
    let key = std::env::var("GEMINI_API_KEY")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| secrets::load_api_key().ok())
        .ok_or_else(|| "Define GEMINI_API_KEY o guarda una clave con --set-api-key".to_string())?;
    let target = std::env::var("TARGET_LANG").unwrap_or_else(|_| "es".to_string());
    let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    let message = runtime.block_on(gemini::test_connection(&key, &target))?;
    println!("{message}");
    Ok(())
}

pub fn cli_set_api_key(key: &str) -> Result<(), String> {
    secrets::save_api_key(key.trim())?;
    println!("API key guardada en el llavero de Windows");
    Ok(())
}

pub fn cli_probe_audio() -> Result<(), String> {
    let (rms, peak) = audio::probe_capture("default", 3)?;
    println!("RMS={rms:.6} PEAK={peak:.6}");
    Ok(())
}
