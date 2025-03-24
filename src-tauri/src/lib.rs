// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod command;
mod data;
mod winx;
use command::{get_win_all, send_text, start_window, stop_window};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(data::AppState::default())
        .setup(|app| {
            // let main_window = app.get_webview_window("main").unwrap();

            // if let Some(current_monitor) = main_window.current_monitor()? {
            //     let screen_size = current_monitor.size();
            //     let screen_height = screen_size.height as f64;

            //     main_window.set_size(tauri::LogicalSize::new(400 as f64, screen_height))?;
            //     // main_window.set_position(tauri::Position::Right)?;
            // }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_win_all,start_window,stop_window,send_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
