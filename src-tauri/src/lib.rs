// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod command;
mod constants;
mod data;
mod utils;
mod winx;
use command::*;
use tauri::{AppHandle, Manager};
use tauri_plugin_autostart::MacosLauncher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = show_window(app);
        }))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .manage(data::AppState::default())
        .setup(|app| {
            let handle = app.handle();
            command::background_task(&handle.clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_window_all,
            window_start,
            window_stop,
            window_send_text,
            background_task_start,
            background_task_stop,
            get_session_id,
            set_session_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn show_window(app: &AppHandle) {
    let windows = app.webview_windows();

    windows
        .values()
        .next()
        .expect("Sorry, no window found")
        .set_focus()
        .expect("Can't Bring Window to Focus");
}
