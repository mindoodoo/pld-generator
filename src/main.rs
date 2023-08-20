pub mod github;
pub mod parsing;

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
    let api_key = lookup_env!("API_KEY");
    let project = ProjectId {
        org: lookup_env!("PROJECT_OWNER"),
        project: lookup_env!("PROJECT_NUM").parse()
            .expect("Error parsing PROJECT_NUM into integer")
    };


    let client = github::ProjectsClient::new(&api_key, project);
    client.get_cards().await;
}
