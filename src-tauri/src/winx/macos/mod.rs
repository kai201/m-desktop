mod api;
use api::MacosAPI;
use super::core::Api;

pub fn init_api() -> impl Api {
    MacosAPI {}
}
