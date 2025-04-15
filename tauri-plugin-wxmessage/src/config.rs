use serde::{Deserialize, Deserializer};
use tauri::Url;

/// Updater configuration.
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// Updater endpoints.
    pub endpoints: Vec<Url>,
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Config {
            #[serde(default)]
            pub endpoints: Vec<Url>,
        }

        let config = Config::deserialize(deserializer)?;

        Ok(Self {
            endpoints: config.endpoints,
        })
    }
}
