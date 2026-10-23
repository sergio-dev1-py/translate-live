const SERVICE: &str = "com.translatelive.app";
const USER: &str = "gemini-api-key";

fn entry() -> Result<keyring::Entry, String> {
    keyring::Entry::new(SERVICE, USER)
        .map_err(|error| format!("No se pudo abrir el llavero del sistema: {error}"))
}

pub fn save_api_key(key: &str) -> Result<(), String> {
    entry()?
        .set_password(key)
        .map_err(|error| format!("No se pudo guardar la API key en el llavero: {error}"))
}

pub fn load_api_key() -> Result<String, String> {
    if let Ok(value) = std::env::var("GEMINI_API_KEY") {
        let value = value.trim().to_string();
        if !value.is_empty() {
            return Ok(value);
        }
    }
    entry()?
        .get_password()
        .map_err(|error| format!("No hay una API key guardada: {error}"))
}

pub fn has_api_key() -> bool {
    load_api_key()
        .map(|key| !key.trim().is_empty())
        .unwrap_or(false)
}
