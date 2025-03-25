mod api;
use super::core::Api;
use api::Win32API;

pub fn init_api() -> impl Api {
    Win32API {}
}
