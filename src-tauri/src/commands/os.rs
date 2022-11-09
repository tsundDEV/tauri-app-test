use dark_light;
use std::env;
use tauri::command;

#[command]
pub async fn get_system() -> String {
    env::consts::OS.to_string()
}

#[command]
pub async fn get_system_theme() -> String {
    match dark_light::detect() {
        dark_light::Mode::Dark => {
            return "dark".into();
        }
        dark_light::Mode::Light => {
            return "light".into();
        }
    }
}
