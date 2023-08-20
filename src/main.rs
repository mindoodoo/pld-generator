pub mod github;
pub mod parsing;

use github::{ProjectId, card};
use parsing::PldCard;

use dotenv::dotenv;
use std::{env, fs};
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

    let card_content = fs::read_to_string("./card_format.md").unwrap();
    let pld_card = PldCard::from_markdown(card_content).expect("Error parsing card");

    println!("{:?}", pld_card);
    // let client = github::ProjectsClient::new(&api_key, project);
    // client.get_cards().await;
}

