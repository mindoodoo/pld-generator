pub mod github;
pub mod lucid;

use github::ProjectId;

use dotenv::dotenv;
use std::env;
use tokio;

// This is mostly just me taking this opportunity to try out macros in rust
macro_rules! lookup_env {
    ($key:expr) => {
        env::vars().find(|(key, _)| key == $key).expect("$key not found in .env file").1
    };
}

#[tokio::main]
async fn main() {
    dotenv().expect("Error loading .env file");
    let api_key = lookup_env!("GITHUB_API_KEY");

    let project = ProjectId {
        org: lookup_env!("PROJECT_OWNER"),
        project: lookup_env!("PROJECT_NUM").parse()
            .expect("Error parsing PROJECT_NUM into integer")
    };

    let gh_client = github::ProjectsClient::new(&api_key, project);
    let lucid_client = lucid::LucidClient::new(
        &lookup_env!("LUCID_ACCESS_TOKEN"),
        &lookup_env!("LUCID_REFRESH_TOKEN"),
        &lookup_env!("LUCID_CLIENT_ID"),
        &lookup_env!("LUCID_CLIENT_SECRET"));

    // gh_client.get_cards().await;
    println!("{:?}", lucid_client.check_access_token(&lookup_env!("LUCID_REFRESH_TOKEN")).await);
}
