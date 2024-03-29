use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct IntrospectBody {
    pub token: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Deserialize)]
pub struct IntrospectOk {
    pub active: bool,
    pub user_id: i64,
    pub client_id: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: i64,
    pub expires: i64,
}

#[derive(Deserialize)]
pub struct IntrospectErr {
    pub active: bool,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum IntrospectResponse {
    Success(IntrospectOk),
    Error(IntrospectErr),
}

#[derive(Serialize)]
pub struct RefreshBody {
    pub refresh_token: String,
    pub client_id: String,
    pub client_secret: String,
    /// Always `refresh_token`
    pub grant_type: String,
}

#[derive(Deserialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub user_id: i64,
    pub client_id: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub expires: i64,
    pub scope: String,
    pub scopes: Vec<String>,
    pub token_type: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDocumentResponse {
    pub page_count: u8,
}
