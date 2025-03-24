mod core;
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod win32;

use core::Api;

#[cfg(target_os = "macos")]
use macos::init_api;

#[cfg(target_os = "windows")]
use win32::init_api;

pub use core::{ActiveWindow, WindowPosition};

pub fn get_active_window() -> ActiveWindow {
    let api = init_api();
    api.get_active_window()
}

pub fn get_windows() -> Vec<ActiveWindow> {
    let api = init_api();
    api.get_windows()
}
pub fn activate(window_id: String) {
    let api = init_api();
    api.activate(window_id);
}
