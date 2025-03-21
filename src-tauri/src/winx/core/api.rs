use super::model::ActiveWindow;
pub trait Api {
    fn get_active_window(&self) -> ActiveWindow;

    fn get_windows(&self) -> Vec<ActiveWindow>;

    fn activate(&self, window_id: String);
}
