use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct IntrospectBody {
    pub token: String,
    pub client_id: String,
    pub client_secret: String
}

#[derive(Deserialize, Serialize)]
pub struct IntrospectOk {
    pub active: bool,
    user_id: i64,
    client_id: String,
    token_type: String,
    scope: String,
    expires_in: i64,
    expires: i64,
}

#[derive(Deserialize)]
pub struct IntrospectErr {
    active: bool
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum IntrospectResponse {
    Success(IntrospectOk),
    Error(IntrospectErr)
}

#[derive(Serialize)]
pub struct RefreshBody {
    pub refresh_token: String,
    pub client_id: String,
    pub client_secret: String,
    /// Always `refresh_token`
    pub grant_type: String
}

#[derive(Deserialize)]
pub struct RefreshResponse {
    pub access_token: String,
    user_id: i64,
    client_id: String,
    pub refresh_token: String,
    expires_in: i64,
    expires: i64,
    scope: String,
    scopes: Vec<String>,
    token_type: String,
}

