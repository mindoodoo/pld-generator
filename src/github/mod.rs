pub mod card;

use card::{ProjectCard, ProjectItems};

use std::collections::HashMap;
use serde::{Serialize};
use reqwest::{ClientBuilder, Client, header::{HeaderMap, HeaderValue}};

const ENDPOINT: &str = "https://api.github.com/graphql";

// Note: Maybe put this somewhere else in the future
// the endCursor will also need to be used too at some point in order to fully support pagination
const CARDS_QUERY: &str = r#"
{
    organization(login: "Autogrower") {
        projectV2(number: 7) {
            items(first: 5) {
                totalCount
                nodes {
                    content {
                        ... on DraftIssue {
                            title
                            bodyText
                        }
                        ... on Issue {
                            title
                            bodyText
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
                }
                pageInfo {
                    endCursor
                }
            }
        }
    }
}
"#;

/// Identifying information for target project
pub struct ProjectId {
    /// Project org owner
    pub org: String,
    /// Project number within the org
    pub project: usize
}

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
    api_key: String,
    project: ProjectId,
    client: Client
}

#[derive(Serialize)]
struct GqlQuery {
    query: String
}

impl ProjectsClient {
    pub fn new(api_key: &str, project: ProjectId) -> ProjectsClient {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());
        headers.insert("User-Agent", HeaderValue::from_static("pld-generator"));

        ProjectsClient {
            api_key: api_key.into(),
            project,
            client: ClientBuilder::new()
                .default_headers(headers)
                .build()
                .expect("Error building reqwest client")
        }
    }

    pub async fn get_cards(&self) -> HashMap<String, Vec<ProjectCard>> {
        let mut output: HashMap<String, Vec<ProjectCard>> = HashMap::new();

        let json_resp: serde_json::Value = self.client.post(ENDPOINT)
            .json(&GqlQuery { query: CARDS_QUERY.into() })
            .send()
            .await.expect("Error sending cards graphql request")
            .json::<serde_json::Value>()
            .await.expect("Error deserializing cards json response");

        let parsed_resp: ProjectItems = serde_json::from_value(
            json_resp["data"]["organization"]["projectV2"]["items"].clone()
        ).expect("Error deserializing json response");

        for card in parsed_resp.nodes {
            output.entry(card.section.clone())
                .or_default()
                .push(card);
        };

        output
    }
}

// Ideas :
// - Maybe include the json part where you parse the nested stuff, inside the deserializer of ProjectItemsResp
// - Remove some small stucts with deserialize_with field attribute

// Left to do