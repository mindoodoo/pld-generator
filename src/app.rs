use std::{path::PathBuf, fs::{self, File}, fmt::Display, io::Write, error::Error};
use colored::Colorize;

use crate::{config::Config, lucid::LucidClient, github::ProjectsClient, parsing::{PldCard, sort_by_section}};

// Tags
const LUCID_TAG: &str = "{{lucid}}";
const CARDS_TAG: &str = "{{cards}}";

#[derive(Debug)]
pub enum GeneratorError {
    InvalidOutputDirectory,
    LucidInvalidRefreshToken,
    TemplateError,
    WriteFailed
}

impl Display for GeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneratorError::InvalidOutputDirectory => write!(f, "Accessing or creating specified output directory").unwrap(),
            GeneratorError::LucidInvalidRefreshToken => write!(f, "The specified lucid refresh token is invalid").unwrap(),
            GeneratorError::TemplateError => write!(f, "The template could not be found").unwrap(),
            GeneratorError::WriteFailed => write!(f, "Writing to the specified output file has failed").unwrap()
        };
        
        Ok(())
    }
}

impl Error for GeneratorError {}

pub struct App {
    output_dir: String,
    output_file: File,
    output_buffer: String,
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
            output_file: File::create(output_file)
                .map_err(|_| GeneratorError::InvalidOutputDirectory)?,
            output_buffer: fs::read_to_string("./template.md")
                .map_err(|_| GeneratorError::TemplateError)?,
            lucid_client: LucidClient::new(
                &conf.lucid.access_token,
                &conf.lucid.refresh_token,
                &conf.lucid.client_id,
                &conf.lucid.client_secret
            ),
            projects_client: ProjectsClient::new(&conf.github.api_key, conf.github.project_number),
            conf,
        })
    }

    /// Checks if lucid token is valid and attempts to update token if not
    async fn ensure_lucid_token_validity(&mut self) -> Result<(), GeneratorError> {
        if !(self.lucid_client.check_access_token(&self.conf.lucid.access_token).await) {
            let (new_access, new_refresh) = self.lucid_client.refresh_token().await
                .map_err(|_| GeneratorError::LucidInvalidRefreshToken)?;

            self.conf.lucid.access_token = new_access;
            self.conf.lucid.refresh_token = new_refresh;
        }

        Ok(())
    }

    /// Downloads all images to the output directory and writes the diagram of the deliverables
    async fn write_images_md(&mut self) {
        let mut image_paths = Vec::new();
        let mut images_buf = Vec::new();

        if self.output_buffer.find(LUCID_TAG).is_none() {
            return;
        }

        let n_pages = self.lucid_client.get_page_count(&self.conf.lucid.document_id).await
            .expect("Error querying document page number lucid chart");

        for page in 1..=n_pages {
            let mut dest = PathBuf::from(&self.output_dir);
            let image_path = format!("images/{}.png", page.to_string());
            dest.push(&image_path);

            self.lucid_client.export_image(dest.to_str().unwrap(), &self.conf.lucid.document_id, page).await
                .expect("Error downloading image");

            image_paths.push(image_path);
        }

        writeln!(images_buf, r#"<p align="center">"#).unwrap();

        let height = match &self.conf.doc.image_height {
            Some(h) => format!("height = {}", h),
            None => "".to_string()
        };

        let width = match &self.conf.doc.image_width {
            Some(w) => format!("width = {}", w),
            None => "".to_string()
        };

        for path in image_paths {
            writeln!(images_buf, "  <img src=\"{}\" {} {}/>\n  <br></br>",
                path, width, height).unwrap();
        }

        write!(images_buf, "</p>").unwrap();
        
        // Append markdown to buffer
        self.output_buffer = self.output_buffer.replace(LUCID_TAG, &String::from_utf8(images_buf).unwrap());
    }

    async fn write_cards(&mut self) {
        let cards: Vec<PldCard> = self.projects_client.get_cards().await.iter()
            .map(|card| {
                let parsed_card = PldCard::new(card);

                if parsed_card.is_err() {
                    println!("{} Skipping card \"{}\" due to parsing failure.", "WARNING: ".yellow(), card.name.yellow());
                }
                
                parsed_card
            })
            .filter(|card| card.is_ok())
            .map(|card| card.unwrap()).collect();
        let sorted_cards = sort_by_section(cards);

        let mut cards_buf = Vec::new();

        for (section_name, sub_section_map) in sorted_cards {
            write!(cards_buf, "<center>\n  <h2>{}</h2>\n</center>\n\n", section_name).unwrap();

            for (subsection_name, sub_section_cards) in sub_section_map.iter() {
                write!(cards_buf, "### {}\n\n<hr style=\"height: 3px\">\n\n", subsection_name).unwrap();

                for (i, card) in sub_section_cards.iter().enumerate() {

                    let separator = if i == sub_section_cards.len() - 1 { "" } else { "<hr style=\"height: 1px\">\n\n" };

                    write!(cards_buf, "#### {}\n\n{}", card, separator).unwrap();
                }
            }
        }

        self.output_buffer = self.output_buffer.replace(CARDS_TAG, &String::from_utf8(cards_buf).unwrap());
    }

    /// Run generator
    pub async fn run(&mut self) -> Result<(), GeneratorError> {
        self.ensure_lucid_token_validity().await?;

        self.write_images_md().await;

        self.write_cards().await;
        
        self.output_file.write(self.output_buffer.as_bytes()).map_err(|_| GeneratorError::WriteFailed)?;

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
