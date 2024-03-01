pub mod card;

use card::{ProjectCard, ProjectItems};

use colored::Colorize;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder, StatusCode,
};
use serde::Serialize;

const ENDPOINT: &str = "https://api.github.com/graphql";

// Note: Maybe put this somewhere else in the future
// the endCursor will also need to be used too at some point in order to fully support pagination
const CARDS_QUERY: &str = r#"
{
    organization(login: "Autogrower") {
        projectV2(number: 18) {
            items(first: 100) {
                totalCount
                nodes {
                    content {
                        ... on DraftIssue {
                            title
                            body
                            assignees(first: 10) {
                                nodes {
                                    login
                                }
                            }
                        }
                        ... on Issue {
                            title
                            body
                            assignees(first: 10) {
                                nodes {
                                    login
                                }
                            }
                        }
                    }
                    working_days: fieldValueByName(name: "Working Days") {
                        ... on ProjectV2ItemFieldNumberValue {
                            number
                        }
                    }
                    section: fieldValueByName(name: "Section") {
                        ... on ProjectV2ItemFieldSingleSelectValue {
                            name
                        }
                    }
                    status: fieldValueByName(name: "Status") {
                        ... on ProjectV2ItemFieldSingleSelectValue {
                            name
                        }
                    }
                    sub_section: fieldValueByName(name: "Sub-Section") {
                        ... on ProjectV2ItemFieldSingleSelectValue {
                            name
                        }
                    }
                }
                pageInfo {
                    endCursor
                }
            }
        }
    }
}
"#;

/// Main client struct for all requests relevent to github projects
pub struct ProjectsClient {
    /// Api key for request authentication
    ///
    /// Permissions required (read) :
    /// - Repo
    ///   - Issues
    ///   - Metadata (required by issues)
    /// - Organization
    ///   - Projects
    ///
    /// Note: The token needs to be a fine grained token as classic tokens
    /// do not work with the github graphQL API
    project: u8,
    client: Client,
}

#[derive(Serialize)]
struct GqlQuery {
    query: String,
}

impl ProjectsClient {
    pub fn new(api_key: &str, project: u8) -> ProjectsClient {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );
        headers.insert("User-Agent", HeaderValue::from_static("pld-generator"));

        ProjectsClient {
            project,
            client: ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    pub async fn get_cards(&self) -> Vec<ProjectCard> {
        let query_str = String::from(CARDS_QUERY)
            .replace("$PROJECT", &self.project.to_string())
            .replace("$CARD_COUNT", &100.to_string());

        let resp = self
            .client
            .post(ENDPOINT)
            .json(&GqlQuery { query: query_str })
            .send()
            .await
            .expect("Error sending cards graphql request");

        let status = resp.status();

        let json_resp: serde_json::Value = resp
            .json::<serde_json::Value>()
            .await
            .expect("Error deserializing cards json response");

        if status != StatusCode::OK {
            println!("{}", "Error while fetching the github cards, this is most likely an authentication issue.".to_string().red());
            println!("Error message : {}", json_resp["message"].to_string());

            return Vec::new();
        }

        let parsed_resp: ProjectItems =
            serde_json::from_value(json_resp["data"]["organization"]["projectV2"]["items"].clone())
                .expect("Error deserializing json response");

        parsed_resp.nodes
    }
}
