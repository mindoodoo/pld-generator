use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub github_api_key: String,
    pub project_number: u8,
    pub lucid_client_id: String,
    pub lucid_client_secret: String,
    pub lucid_access_token: String,
    pub lucid_refresh_token: String,
    pub document_id: String,
    pub image_width: Option<String>,
    pub image_height: Option<String>,
    /// Path from which the config was loaded
    #[serde(skip)]
    pub path: String
}