use tauri::{Manager, Window};

#[tauri::command]
pub fn close_splashscreen(window: Window) {
    match window.get_window("splashscreen") {
        Some(window) => window.close().unwrap(),
        None => return,
    }

    match window.get_window("main") {
        Some(window) => window.show().unwrap(),
        None => (),
    }
}
