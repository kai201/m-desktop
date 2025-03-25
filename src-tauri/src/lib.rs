// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod command;
mod constants;
mod data;
mod utils;
mod winx;
use command::*;
use tauri_plugin_autostart::MacosLauncher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(data::AppState::default())
        .setup(|_app| {
            // let main_window = app.get_webview_window("main").unwrap();

            // if let Some(current_monitor) = main_window.current_monitor()? {
            //     let screen_size = current_monitor.size();
            //     let screen_height = screen_size.height as f64;

            //     main_window.set_size(tauri::LogicalSize::new(400 as f64, screen_height))?;
            //     // main_window.set_position(tauri::Position::Right)?;
            // }
            println!("{}", constants::API_URL);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_window_all,
            window_start,
            window_stop,
            send_text,
            background_task_start,
            background_task_stop
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
