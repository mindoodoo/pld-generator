mod github;
mod parsing;
mod lucid;
mod app;
mod config;
mod image_cropping;

use app::App;
use clap::Parser;
use config::Config;
use std::{fs::File, io::Read, error::Error};
use tokio;

#[derive(Parser, Debug)]
/// A simple epitech project log document generator
struct Args {
    /// Output directory path. Will be created if does not exist already.
    #[arg(short, long)]
    pub output: String,
    /// Alternative config file path, if unset will default to ./generator_config.toml
    #[arg(short, long)]
    pub conf: Option<String>
}

fn parse_config(path: &str) -> Option<Config> {
    let mut file = File::open(path).ok()?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).ok()?;

    let mut config: Config = toml::from_str(&file_content).unwrap();
    config.path = path.to_string();

    Some(config)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    let conf = match args.conf {
        Some(path) => parse_config(&path),
        None => parse_config("./generator_config.toml")
    }.ok_or("Configuration parsing failed")?;

    let mut app = App::new(conf, &args.output)?;
    app.run().await?;

    Ok(())
}
