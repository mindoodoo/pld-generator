use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().expect("Error loading .env file");
    let api_key = env::vars().find(|(key, _)| key == "API_KEY").expect("API_KEY not found in .env file").1;
}
