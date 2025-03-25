mod api;
use super::core::Api;
use api::MacosAPI;

pub fn init_api() -> impl Api {
    MacosAPI {}
}
