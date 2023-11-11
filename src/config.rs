use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubConfig {
    pub api_key: String,
    pub project_number: u8
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LucidConfig {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub refresh_token: String,
    pub document_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentSettings {
    pub image_width: Option<String>,
    pub image_height: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub github: GithubConfig,
    pub lucid: Option<LucidConfig>,
    #[serde(rename = "document-settings")]
    pub doc: DocumentSettings,
    #[serde(skip)]
    pub path: String
}
