mod model;

use reqwest::{Client, header::{HeaderMap, HeaderValue}, ClientBuilder, StatusCode, Url};
use std::{fs::File, io::{Cursor, copy}};

use model::{IntrospectResponse, IntrospectBody, GetDocumentResponse};

use self::model::{RefreshBody, RefreshResponse};

const API_VERSION: &str = "1";
const INTROSPECT_TOKEN_ROUTE: &str = "https://api.lucid.co/oauth2/token/introspect";
const REFRESH_TOKEN_ROUTE: &str = "https://api.lucid.co/oauth2/token";
const EXPORT_DOCUMENT_ROUTE: &str = "https://api.lucid.co/documents/";
const GET_DOCUMENT_ROUTE: &str = "https://api.lucid.co/documents/";

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
    pub async fn refresh_token(&mut self) -> Result<(String, String), LucidError> {
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

        Ok((self.access_token.clone(), self.refresh_token.clone()))
    }

    /// Export cropped png image
    pub async fn export_image(&self, destination: &str, document_id: &str, page: u8) -> Result<(), LucidError> {
        let page_str = page.to_string();
        let params = Vec::from([
            ("page", page_str.as_ref()),
            ("crop", "content")
        ]);

        let mut query_string = Url::parse_with_params(EXPORT_DOCUMENT_ROUTE, params).unwrap();
        query_string.set_path(&format!("/documents/{}", document_id));
        
        let resp = self.client.get(query_string)
            .header("Accept", "image/png")
            .header("Authorization", &format!("Bearer {}", self.access_token))
            .send().await.unwrap();

        match resp.status() {
            StatusCode::OK => {
                let mut f = File::create(destination).unwrap();
                let mut writer = Cursor::new(resp.bytes().await.unwrap());
                copy(&mut writer, &mut f).expect("Error copying image to file");

                Ok(())
            },
            StatusCode::UNAUTHORIZED => Err(LucidError::ExpiredToken),
            _ => Err(LucidError::UnexpectedResponse)
        }
    }

    pub async fn get_page_count(&self, document_id: &str) -> Result<u8, LucidError> {
        let query_str = format!("{}{}", GET_DOCUMENT_ROUTE, document_id);
        
        let resp = self.client.get(&query_str)
            .header("Authorization", &format!("Bearer {}", self.access_token))
            .header("Accept", "application/json")
            .header("Lucid-Api-Version", "1")
            .send().await.unwrap();

        match resp.status() {
            StatusCode::OK => {
                let body: GetDocumentResponse = resp.json().await.expect("Deserialization failed");

                Ok(body.page_count)
            },
            StatusCode::UNAUTHORIZED => Err(LucidError::ExpiredToken),
            _ => Err(LucidError::UnexpectedResponse)
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
