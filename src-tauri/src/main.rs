use tauri::{App, Builder, Wry};

mod commands;
mod config;
mod utils;
// mod events;

fn main() {
    let app: App = {
        let builder: Builder<Wry> = tauri::Builder::default();
        builder
    }
    .setup(|_app| {
        config::init();
        Ok(())
    })
    // .on_window_event(|window| events::init(window))
    .invoke_handler(commands::init())
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .build(tauri::generate_context!())
    .expect("error while running tauri application");
    app.run(utils::app_events::handle_run_events);
}
