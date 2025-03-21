use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct WindowPosition {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct ActiveWindow {
    pub title: String,
    pub app_name: String,
    pub exec_name: String,
    pub window_id: String,
    pub process_id: u32,
    pub memory: u32,
    pub position: WindowPosition,
}

impl ActiveWindow {
    pub fn empty() -> Self {
        Self {
            title: "".to_string(),
            app_name: "".to_string(),
            exec_name: "".to_string(),
            window_id: "".to_string(),
            process_id: 0,
            memory: 0,
            position: Default::default(),
        }
    }
}

impl Default for WindowPosition {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl WindowPosition {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            x,
            y,
            width: w,
            height: h,
        }
    }
}
