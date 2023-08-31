pub mod github;
pub mod parsing;
pub mod lucid;

use github::{ProjectId, card};
use parsing::PldCard;

use dotenv::dotenv;
use std::{env, fs};
use tokio;

#[tokio::main]
async fn main() {
    dotenv().expect("Error loading .env file");
    let api_key = env::var("GITHUB_API_KEY").unwrap();

    let project = ProjectId {
        org: env::var("PROJECT_OWNER").unwrap(),
        project: env::var("PROJECT_NUM").unwrap().parse()
            .expect("Error parsing PROJECT_NUM into integer")
    };

    let gh_client = github::ProjectsClient::new(&api_key, project);
    let mut lucid_client = lucid::LucidClient::new(
        &env::var("LUCID_ACCESS_TOKEN").unwrap(),
        &env::var("LUCID_REFRESH_TOKEN").unwrap(),
        &env::var("LUCID_CLIENT_ID").unwrap(),
        &env::var("LUCID_CLIENT_SECRET").unwrap());
}

