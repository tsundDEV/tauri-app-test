use tauri::{AppHandle, RunEvent, Wry};
pub fn handle_run_events(_app_handle: &AppHandle<Wry>, e: RunEvent) {
    match e {
        RunEvent::Exit => {}
        RunEvent::ExitRequested { .. } => {}
        RunEvent::WindowEvent {
            label: _, event: _, ..
        } => {}
        RunEvent::Ready => {}
        RunEvent::Resumed => {}
        RunEvent::MainEventsCleared => {}
        _ => {}
    }
}
