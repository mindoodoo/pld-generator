use std::{path::{Path, PathBuf}, fs::{self, File, OpenOptions}, fmt::Display, io::Write, error::Error};

use crate::{Args, config::Config, lucid::LucidClient, github::ProjectsClient};

const DOC_BEGIN: &str = "# Project Log Document

## Revision Table

## Document Description

## Table of Revisions


";

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
    output_file: File,
    conf: Config,
    lucid_client: LucidClient,
    projects_client: ProjectsClient
}

impl App {
    pub fn new(conf: Config, output_dir: &str) -> Result<Self, GeneratorError> {
        fs::create_dir_all(format!("{}/{}", output_dir, "images")).map_err(|_| GeneratorError::InvalidOutputDirectory)?;
        let output_file = PathBuf::from(format!("{}/pld.md", output_dir));

        {
            // Ensure that the file is empty
            let f = File::create(&output_file).map_err(|_| GeneratorError::InvalidOutputDirectory)?;
            f.set_len(0).unwrap();
        }

        Ok(App {
            output_dir: output_dir.to_string(),
            lucid_client: LucidClient::new(
                &conf.lucid_access_token,
                &conf.lucid_refresh_token,
                &conf.lucid_client_id,
                &conf.lucid_client_secret
            ),
            projects_client: ProjectsClient::new(&conf.github_api_key, conf.project_number),
            conf,
            output_file: OpenOptions::new().append(true).open(output_file)
                .map_err(|_| GeneratorError::InvalidOutputDirectory)?
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

    /// Downloads all images to the output directory and writes the diagram of the deliverables
    async fn write_images(&mut self) {
        let mut image_paths = Vec::new();

        let n_pages = self.lucid_client.get_page_count(&self.conf.document_id).await
            .expect("Error querying document page number lucid chart");

        for page in 1..=n_pages {
            let mut dest = PathBuf::from(&self.output_dir);
            let image_path = format!("images/{}.png", page.to_string());
            dest.push(&image_path);

            self.lucid_client.export_image(dest.to_str().unwrap(), &self.conf.document_id, page).await
                .expect("Error downloading image");

            image_paths.push(image_path);
        }

        write!(self.output_file, "## Diagram of the Deliverables\n\n").unwrap();
        write!(self.output_file, r#"<p align="center">"#).unwrap();

        for path in image_paths {
            writeln!(self.output_file, r#"  <img src="{}" />"#, path).unwrap();
        }

        write!(self.output_file, "</p>\n\n").unwrap();
    }

    /// Run generator
    pub async fn run(&mut self) -> Result<(), GeneratorError> {
        self.ensure_lucid_token_validity().await?;

        write!(self.output_file, "{}", DOC_BEGIN).unwrap();

        self.write_images().await;

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
