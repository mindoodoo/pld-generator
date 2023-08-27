mod model;

use reqwest::{Client, header::{HeaderMap, HeaderValue}, ClientBuilder, StatusCode};
use serde::{Serialize, Deserialize};

use model::{IntrospectResponse, IntrospectBody};

use self::model::{RefreshBody, RefreshResponse};

const API_VERSION: &str = "1";
const INTROSPECT_TOKEN_ROUTE: &str = "https://api.lucid.co/oauth2/token/introspect";
const REFRESH_TOKEN_ROUTE: &str = "https://api.lucid.co/oauth2/token";

pub struct OauthId {
    client_id: String,
    client_secret: String
}

/// Main client structure for all requests relevent to lucid chart
pub struct LucidClient {
    client: Client,
    pub access_token: String,
    pub refresh_token: String,
    oauth_id: OauthId
}

#[derive(Debug)]
pub enum LucidError {
    ExpiredToken,
    UnexpectedResponse
}

impl LucidClient {
    /// Introspects given token, returns true if token is still valid
    /// 
    /// Note : Has weird behavious with refresh_token so use it exclusively for access_token
    pub async fn check_access_token(&self, token: &str) -> bool {
        let body = IntrospectBody {
            client_id: self.oauth_id.client_id.clone(),
            client_secret: self.oauth_id.client_secret.clone(),
            token: token.to_string()
        };

        let res: IntrospectResponse = self.client.post(INTROSPECT_TOKEN_ROUTE)
            .json(&body)
            .send()
            .await.expect("Error sending access token introspection request")
            .json()
            .await.expect("Error deserializing access token introspection response");

        match res {
            IntrospectResponse::Success(res) => res.active,
            IntrospectResponse::Error(_) => false
        }
    }

    /// Refreshes both refresh token and access_token
    pub async fn refresh_token(&mut self) -> Result<(), LucidError> {
        let body = RefreshBody {
            refresh_token: self.refresh_token.clone(),
            client_id: self.oauth_id.client_id.clone(),
            client_secret: self.oauth_id.client_secret.clone(),
            grant_type: String::from("refresh_token")
        };

        let res = self.client.post(REFRESH_TOKEN_ROUTE)
            .json(&body)
            .send()
            .await.expect("Error sending access token introspection request");
        
        match res.status() {
            StatusCode::UNAUTHORIZED => return Err(LucidError::ExpiredToken),
            StatusCode::OK => (),
            _ => return Err(LucidError::UnexpectedResponse)
        };

        let res: RefreshResponse = res.json()
            .await.expect("Error deserializing refresh token response.
                This most likely means that the refresh token used is invalid or has expired");
        
        self.access_token = res.access_token;
        self.refresh_token = res.refresh_token;

        Ok(())
    }

    pub fn new(access_token: &str, refresh_token: &str, client_id: &str, client_secret: &str) -> LucidClient {
        let mut headers = HeaderMap::with_capacity(1);
        headers.insert("Lucid-Api-Version", HeaderValue::from_static(API_VERSION));

        LucidClient {
            client: ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap(),
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
            oauth_id: OauthId {
                client_id: client_id.to_string(),
                client_secret: client_secret.to_string()
            }
        }
    }
}
