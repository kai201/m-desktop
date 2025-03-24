
pub struct Win32API {}

use crate::winx::core::{ActiveWindow, Api, WindowPosition};
impl Api for Win32API {
    fn get_active_window(&self) -> ActiveWindow {
        todo!()
    }

    fn get_windows(&self) -> Vec<ActiveWindow> {
        todo!()
    }

    fn activate(&self, window_id: String) {
        todo!()
    }
}


// fn get_foreground_window() -> HWND {
//     unsafe { GetForegroundWindow() }
// }