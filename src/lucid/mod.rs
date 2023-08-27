mod model;

use reqwest::{Client, header::{HeaderMap, HeaderValue}, ClientBuilder};
use serde::{Serialize, Deserialize};

use model::{IntrospectResponse, IntrospectBody};

const API_VERSION: &str = "1";
const INTROSPECT_TOKEN_ROUTE: &str = "https://api.lucid.co/oauth2/token/introspect";

pub struct OauthId {
    client_id: String,
    client_secret: String
}

/// Main client structure for all requests relevent to lucid chart
pub struct LucidClient {
    client: Client,
    access_token: String,
    refresh_token: String,
    oauth_id: OauthId
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

        let res = self.client.post(INTROSPECT_TOKEN_ROUTE)
            .json(&body)
            .send()
            .await.expect("Error sending access token introspection request")
            .json::<IntrospectResponse>()
            .await.expect("Error deserializing access token introspection response");

        match res {
            IntrospectResponse::Success(res) => res.active,
            IntrospectResponse::Error(_) => false
        }
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
