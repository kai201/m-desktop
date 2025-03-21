mod core;
mod macos;

use core::{ActiveWindow, Api};

use macos::init_api;

pub fn get_active_window() {
    let api = init_api();
    println!("get_active_window");
    api.get_active_window();
}

pub fn get_windows() -> Vec<ActiveWindow> {
    let api = init_api();
    println!("get_windows");
    api.get_windows()
}
pub fn activate(window_id: String) {
    let api = init_api();
    api.activate(window_id);
}
