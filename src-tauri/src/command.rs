use std::thread;
use std::time::Duration;

use enigo::{Enigo, Keyboard, Settings};
use std::sync::atomic::Ordering;
use tauri::{AppHandle, Manager};

use crate::winx::ActiveWindow;
use crate::{
    data::AppState,
    winx::{activate, get_active_window, get_windows},
};

#[tauri::command]
pub fn start_window(app: AppHandle) {
    let state = app.state::<AppState>();

    if state.is_capture.load(Ordering::Relaxed) {
        return; // 避免重复启动
    }
    println!("start");
    state.is_capture.store(true, Ordering::Relaxed);

    let flag = state.is_capture.clone();
    let win = state.window.clone();
    let exe_list = vec![
        String::from("微信"),
        String::from("WeChat"),
        String::from("企业微信"),
        String::from("钉钉"),
    ];

    thread::spawn(move || {
        while flag.load(Ordering::Relaxed) {
            let active_window = get_active_window();

            if std::process::id() == active_window.process_id {
                thread::sleep(Duration::from_millis(200)); // 每秒检查一次
                continue;
            }
            println!(
                "Name: {}, Title: {},WinName:{},HW:{},X:{},Y:{},H:{},W:{}",
                active_window.app_name,
                active_window.title,
                active_window.win_name,
                active_window.window_id,
                active_window.position.x,
                active_window.position.y,
                active_window.position.height,
                active_window.position.width
            );

            let app_name = active_window.app_name.clone();

            if !exe_list.contains(&app_name) {
                thread::sleep(Duration::from_millis(200)); // 每秒检查一次

                *win.lock().unwrap() = None;
                continue;
            }

            *win.lock().unwrap() = Some(active_window);

            thread::sleep(Duration::from_millis(600)); // 每秒检查一次
        }
    });
}

#[tauri::command]
pub fn stop_window(app: AppHandle) {
    let state = app.state::<AppState>();
    state.is_capture.store(false, Ordering::Relaxed);
}

#[tauri::command]
pub fn get_win_all() -> Vec<ActiveWindow> {
    println!("{}", std::process::id());
    get_windows()
}

#[tauri::command]
pub fn send_text(app: AppHandle, txt: String) {
    let state = app.state::<AppState>();
    let mut guard = state.window.lock().unwrap();
    if let Some(win) = guard.take() {
        #[cfg(target_os = "macos")]
        activate(win.process_id.to_string());
        #[cfg(target_os = "windows")]
        activate(win.window_id.clone());
        thread::sleep(Duration::from_millis(500));
        let mut opts = Settings::default();
        opts.open_prompt_to_get_permissions = false;
        if let Ok(mut enigo) = Enigo::new(&opts) {
            enigo
                .text(&txt)
                .map_err(|e| format!("鼠标操作失败: {}", e))
                .unwrap();
        }
    }
}
