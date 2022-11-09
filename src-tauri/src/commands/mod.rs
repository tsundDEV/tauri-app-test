pub mod os;

pub fn init() -> Box<dyn Fn(tauri::Invoke<tauri::Wry>) + Send + Sync> {
    Box::new(tauri::generate_handler![
        os::get_system,
        os::get_system_theme
    ])
}
