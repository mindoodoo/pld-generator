use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct IntrospectBody {
    pub token: String,
    pub client_id: String,
    pub client_secret: String
}

#[derive(Deserialize)]
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