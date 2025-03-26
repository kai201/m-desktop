// 生产环境常量
#[cfg(feature = "prod")]
pub const API_URL: &str = "https://api.example.com";

// 开发环境常量
#[cfg(feature = "dev")]
pub const API_URL: &str = "http://wxplus.s.test.meb.im";

// 测试环境常量
#[cfg(feature = "test")]
pub const API_URL: &str = "http://test.local";

pub const SESSION_ID: &str = "session_id";
