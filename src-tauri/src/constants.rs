// 生产环境常量
#[cfg(feature = "prod")]
pub const API_URL: &str = "https://api.example.com";

// 开发环境常量
#[cfg(feature = "dev")]
pub const API_URL: &str = "http://localhost:3000";

// 测试环境常量
#[cfg(feature = "test")]
pub const API_URL: &str = "http://test.local";
