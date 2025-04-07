use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use enigo::{Enigo, Keyboard, Settings};
use std::sync::atomic::Ordering;
use tauri::{AppHandle, Emitter, Manager};

use crate::utils;
use crate::winx::ActiveWindow;
use crate::{
    data::AppState,
    winx::{activate, get_active_window, get_windows},
};

use crate::constants;

// 常量定义
const CHECK_INTERVAL: Duration = Duration::from_millis(200);
const CAPTURE_INTERVAL: Duration = Duration::from_millis(600);
// const TARGET_APPS: [&str; 4] = ["微信", "WeChat", "企业微信", "钉钉"];

pub fn background_task(app: &AppHandle) {
    let binding = app.clone();
    let state = binding.state::<AppState>();
    let flag = state.is_running.clone();
    let win = state.window.clone();
    let data = state.data.clone();

    tauri::async_runtime::spawn_blocking(move || loop {
        if !flag.load(Ordering::Relaxed) {
            thread::sleep(CHECK_INTERVAL);
            continue;
        }
        let active_window = get_active_window();
        if std::process::id() == active_window.process_id {
            thread::sleep(CHECK_INTERVAL);
            continue;
        }

        let vmap = data.lock().unwrap();
        let v: Vec<&str> = vmap
            .get(constants::APP_LIST)
            .map_or("微信,WeChat", |v| v)
            .split(",")
            .collect();
        let app_name = active_window.app_name.clone();

        if !v.contains(&app_name.as_str()) {
            binding.emit("capture", ActiveWindow::default()).unwrap();
            *win.lock().unwrap() = None;
            thread::sleep(CHECK_INTERVAL);
            continue;
        }

        binding.emit("capture", active_window.clone()).unwrap();
        *win.lock().unwrap() = Some(active_window);
        thread::sleep(CAPTURE_INTERVAL);
    });
}

#[tauri::command]
pub fn window_start(app: AppHandle) {
    let state = app.state::<AppState>();

    if state.is_running.load(Ordering::Relaxed) {
        return; // 避免重复启动
    }
    state.is_running.store(false, Ordering::Relaxed); 
}

#[tauri::command]
pub fn window_stop(app: AppHandle) {
    let state = app.state::<AppState>();
    state.is_running.store(false, Ordering::Relaxed);
}

#[tauri::command]
pub fn get_window_all() -> Vec<ActiveWindow> {
    println!("{}", std::process::id());
    get_windows()
}

#[tauri::command]
pub fn window_send_text(app: AppHandle, txt: String, send: bool) -> bool {
    let state = app.state::<AppState>();
    let mut guard = state.window.lock().unwrap();

    if let Some(win) = guard.take() {
        #[cfg(target_os = "macos")]
        activate(win.process_id.to_string());
        #[cfg(target_os = "windows")]
        activate(win.window_id.clone());
        thread::sleep(Duration::from_millis(500));
        let mut opts = Settings::default();
        opts.open_prompt_to_get_permissions = true;
        if let Ok(mut enigo) = Enigo::new(&opts).map_err(|e| format!("初始化 Enigo 失败: {}", e))
        {
            enigo
                .text(&txt)
                .map_err(|e| format!("输入文本失败: {}", e))
                .unwrap();
            if send {
                let _ = enigo.key(enigo::Key::Return, enigo::Direction::Click);
            }
            return true;
        }
    }
    false
}

#[tauri::command]
pub async fn background_task_start(app: AppHandle) -> bool {
    let binding = app.state::<AppState>();
    let data = binding.data.clone();
    let background_task_cli = binding.background_task.clone();

    let session_id = {
        let vmap = data.lock().unwrap();
        vmap.get(constants::SESSION_ID).cloned()
    };

    if session_id.is_none() {
        return false;
    }
    let mut docs = app.path().document_dir().unwrap();

    let vers_url = format!(
        "{}/sys/plus/version/{}/{}",
        constants::API_URL,
        std::env::consts::OS.to_string(),
        std::env::consts::ARCH.to_string()
    );

    let result = utils::get_json(&vers_url).await.unwrap_or(HashMap::new());

    let version = result.get("version").map_or("0.0.0", |v| v);
    let download_url = result.get("download_url").map_or("", |v| v);

    if download_url.is_empty() {
        return false;
    }

    #[cfg(target_os = "macos")]
    docs.push(format!("cli-{}", version));
    #[cfg(target_os = "windows")]
    docs.push(format!("cli-{}.exe", version));

    let program = format!("{}", docs.to_str().unwrap());

    if let Ok(s) = utils::download(download_url, &program).await {
        if !s {
            return false;
        }

        let child = Command::new(program)
            .args(["-u", &session_id.unwrap()])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        if let Ok(n) = child {
            *background_task_cli.lock().unwrap() = Some(n);
            return true;
        }
    }
    false
}

#[tauri::command]
pub fn background_task_stop(app: AppHandle) -> bool {
    let binding = app.state::<AppState>();
    let background_task = binding.background_task.clone();

    let mut background_task_cli = background_task.lock().unwrap();

    if let Some(mut cli) = background_task_cli.take() {
        cli.kill().unwrap();
    }

    *background_task_cli = None;

    true
}

#[tauri::command]
pub fn set_session_id(app: AppHandle, session_id: String) {
    let state = app.state::<AppState>();

    let mut data = state.data.lock().unwrap();
    data.insert(constants::SESSION_ID.to_string(), session_id);
}

#[tauri::command]
pub fn get_session_id(app: AppHandle) -> String {
    let state = app.state::<AppState>();

    let data = state.data.lock().unwrap();

    if let Some(session_id) = data.get(constants::SESSION_ID) {
        return String::from(session_id);
    }
    String::from("")
}
