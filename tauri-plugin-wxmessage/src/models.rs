use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerVersion {
    pub version: String,
    pub download_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrrentVersion {
    pub version: String,
    pub executable_path: String,
}
