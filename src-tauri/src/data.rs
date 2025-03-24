use std::{
    collections::HashMap,
    process::Child,
    sync::{atomic::AtomicBool, Arc, Mutex},
};

use crate::winx::ActiveWindow;

pub struct AppState {
    pub is_capture: Arc<AtomicBool>,
    pub data: Arc<Mutex<HashMap<String, String>>>, // 线程安全的字典
    pub window: Arc<Mutex<Option<ActiveWindow>>>,
    pub wxplus: Arc<Mutex<Option<Child>>>,
}

impl AppState {
    pub fn default() -> Self {
        AppState {
            is_capture: Arc::new(AtomicBool::new(false)),
            data: Arc::new(Mutex::new(HashMap::new())),
            window: Arc::new(Mutex::new(None)),
            wxplus: Arc::new(Mutex::new(None)),
        }
    }
}
