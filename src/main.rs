pub mod github;
pub mod parsing;
pub mod lucid;

use dotenv::dotenv;
use parsing::PldCard;
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    dotenv().expect("Error loading .env file");
    let api_key = env::var("GITHUB_API_KEY").unwrap();
    let project_num = env::var("PROJECT_NUM").unwrap().parse().unwrap();

    let gh_client = github::ProjectsClient::new(&api_key, project_num);
    let mut lucid_client = lucid::LucidClient::new(
        &env::var("LUCID_ACCESS_TOKEN").unwrap(),
        &env::var("LUCID_REFRESH_TOKEN").unwrap(),
        &env::var("LUCID_CLIENT_ID").unwrap(),
        &env::var("LUCID_CLIENT_SECRET").unwrap());
    let cards: Vec<PldCard> = gh_client.get_cards().await
        .iter().map(|card| PldCard::new(card).unwrap()).collect();
    println!("{}", cards[0]);
}

