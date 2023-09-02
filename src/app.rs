use std::{path::{Path, PathBuf}, fs::{self, File}, fmt::Display, io::Write, error::Error};

use crate::{Args, config::Config, lucid::LucidClient, github::ProjectsClient};

#[derive(Debug)]
pub enum GeneratorError {
    InvalidOutputDirectory,
    LucidInvalidRefreshToken
}

impl Display for GeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneratorError::InvalidOutputDirectory => write!(f, "Accessing or creating specified output directory").unwrap(),
            GeneratorError::LucidInvalidRefreshToken => write!(f, "The specified lucid refresh token is invalid").unwrap()
        };
        
        Ok(())
    }
}

impl Error for GeneratorError {}

pub struct App {
    output_dir: String,
    conf: Config,
    lucid_client: LucidClient,
    projects_client: ProjectsClient
}

impl App {
    pub fn new(conf: Config, output_dir: &str) -> Result<Self, GeneratorError> {
        fs::create_dir_all(format!("{}/{}", output_dir, "images")).map_err(|_| GeneratorError::InvalidOutputDirectory)?;

        Ok(App {
            output_dir: output_dir.to_string(),
            lucid_client: LucidClient::new(
                &conf.lucid_access_token,
                &conf.lucid_refresh_token,
                &conf.lucid_client_id,
                &conf.lucid_client_secret
            ),
            projects_client: ProjectsClient::new(&conf.github_api_key, conf.project_number),
            conf
        })
    }

    /// Checks if lucid token is valid and attempts to update token if not
    async fn ensure_lucid_token_validity(&mut self) -> Result<(), GeneratorError> {
        if !(self.lucid_client.check_access_token(&self.conf.lucid_access_token).await) {
            let (new_access, new_refresh) = self.lucid_client.refresh_token().await
                .map_err(|_| GeneratorError::LucidInvalidRefreshToken)?;

            self.conf.lucid_access_token = new_access;
            self.conf.lucid_refresh_token = new_refresh;
        }

        Ok(())
    }

    async fn download_images(&self) -> Vec<PathBuf> {
        let mut output = Vec::new();

        let n_pages = self.lucid_client.get_page_count(&self.conf.document_id).await
            .expect("Error querying document page number lucid chart");

        for page in 1..=n_pages {
            let mut dest = PathBuf::from(&self.output_dir);
            dest.push("images");
            dest.push(format!("{}.png", page.to_string()));

            self.lucid_client.export_image(dest.to_str().unwrap(), &self.conf.document_id, page).await
                .expect("Error downloading image");

            output.push(dest);
        }

        output
    }

    pub async fn run(&mut self) -> Result<(), GeneratorError> {
        self.ensure_lucid_token_validity().await?;

        self.download_images().await;

        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let mut file = File::create(&self.conf.path)
            .unwrap();
        file.write_all(toml::to_string_pretty(&self.conf).unwrap().as_bytes()).unwrap();
    }
}
