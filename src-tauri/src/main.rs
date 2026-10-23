#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg| arg == "--test-gemini") {
        match live_translate_lib::cli_test_gemini() {
            Ok(()) => return,
            Err(error) => {
                eprintln!("ERROR: {error}");
                std::process::exit(1);
            }
        }
    }

    if args.iter().any(|arg| arg == "--probe-audio") {
        match live_translate_lib::cli_probe_audio() {
            Ok(()) => return,
            Err(error) => {
                eprintln!("ERROR: {error}");
                std::process::exit(1);
            }
        }
    }

    if let Some(index) = args.iter().position(|arg| arg == "--set-api-key") {
        let key = args
            .get(index + 1)
            .cloned()
            .or_else(|| std::env::var("GEMINI_API_KEY").ok())
            .unwrap_or_default();
        match live_translate_lib::cli_set_api_key(&key) {
            Ok(()) => return,
            Err(error) => {
                eprintln!("ERROR: {error}");
                std::process::exit(1);
            }
        }
    }

    live_translate_lib::run()
}
