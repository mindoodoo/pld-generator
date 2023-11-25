use std::{path::PathBuf, fs::{self, File}, fmt::Display, io::Write, error::Error};
use colored::Colorize;
use regress::{Regex, Flags};

use crate::{config::Config, lucid::LucidClient, github::ProjectsClient, parsing::{PldCard, sort_by_section}, image_cropping::crop_image};

// Tags
const LUCID_TAG: &str = "{{lucid}}";
const CARDS_TAG: &str = "{{cards}}";
const TOC_TAG: &str = "{{table_of_contents}}";

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
    lucid_client: Option<LucidClient>,
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
            lucid_client: if let Some(lucid_conf) = conf.lucid.as_ref() {
                Some(LucidClient::new(
                    &lucid_conf.access_token,
                    &lucid_conf.refresh_token,
                    &lucid_conf.client_id,
                    &lucid_conf.client_secret
                ))
            } else { None },
            projects_client: ProjectsClient::new(&conf.github.api_key, conf.github.project_number),
            conf,
        })
    }

    /// Checks if lucid token is valid and attempts to update token if not
    async fn ensure_lucid_token_validity(&mut self) -> Result<(), GeneratorError> {
        // Any lucid related functions should not be called if lucid conf or lucid client is None
        let lucid_conf = self.conf.lucid.as_mut().unwrap();
        let lucid_client = self.lucid_client.as_mut().unwrap();

        if !(lucid_client.check_access_token(&lucid_conf.access_token).await) {
            let (new_access, new_refresh) = lucid_client.refresh_token().await
                .map_err(|_| GeneratorError::LucidInvalidRefreshToken)?;

            lucid_conf.access_token = new_access;
            lucid_conf.refresh_token = new_refresh;
        }

        Ok(())
    }

    /// Downloads all images to the output directory and writes the diagram of the deliverables
    async fn write_images_md(&mut self) {
        // Any lucid related functions should not be called if lucid conf or lucid client is None
        let lucid_conf = self.conf.lucid.as_mut().unwrap();
        let lucid_client = self.lucid_client.as_mut().unwrap();
        
        let mut image_paths = Vec::new();
        let mut images_buf = Vec::new();

        if self.output_buffer.find(LUCID_TAG).is_none() {
            return;
        }

        let n_pages = lucid_client.get_page_count(&lucid_conf.document_id).await
            .expect("Error querying document page number lucid chart");

        for page in 1..=n_pages {
            let mut dest = PathBuf::from(&self.output_dir);
            let image_path = format!("images/{}.png", page.to_string());
            dest.push(&image_path);

            lucid_client.export_image(dest.to_str().unwrap(), &lucid_conf.document_id, page).await
                .expect("Error downloading image");
            crop_image(&dest);

            image_paths.push(dest);
        }

        writeln!(images_buf, r#"<p align="center">"#).unwrap();

        let (width, height) = if self.conf.doc.is_some() {
            let doc_settings = self.conf.doc.as_ref().unwrap();

            let height = match &doc_settings.image_height {
                Some(h) => format!("height = {}", h),
                None => "".to_string()
            };
    
            let width = match &doc_settings.image_width {
                Some(w) => format!("width = {}", w),
                None => "".to_string()
            };

            (width, height)
        } else { ("".to_string(), "".to_string()) };

        for path in image_paths {
            writeln!(images_buf, "  <img src=\"{}\" {} {}/>\n  <br></br>",
                path.to_string_lossy(), width, height).unwrap();
        }

        write!(images_buf, "</p>").unwrap();
        
        // Append markdown to buffer
        self.output_buffer = self.output_buffer.replace(LUCID_TAG, &String::from_utf8(images_buf).unwrap());
    }

    async fn write_cards(&mut self) {
        let cards: Vec<PldCard> = self.projects_client.get_cards().await.iter()
            .filter_map(|card| {
                if card.working_days == 0.0 {
                    return None;
                }

                match PldCard::new(card) {
                    Err(_) => {
                        println!("{}Skipping card \"{}\" due to parsing failure.", "WARNING: ".yellow(), card.name.yellow());

                        None
                    },
                    Ok(x) => Some(x)
                }
            }).collect();
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

    fn write_table_of_contents(&mut self) {
        const MD_HEADER_REGEX: &str = r"^#+\s.*$";
        const FLAGS: Flags = Flags {
            icase: true,
            multiline: true,
            dot_all: false,
            no_opt: false,
            unicode: false
        };

        let headers_regex = Regex::with_flags(MD_HEADER_REGEX, FLAGS).unwrap();

        let toc_items: Vec<String> = headers_regex.find_iter(&self.output_buffer).filter_map(|m| {
            let match_str = &self.output_buffer[m.range];

            let header_level = match_str.chars().take_while(|c| *c == '#' && *c != '\n').count();
            let match_str = &match_str[header_level..].trim();

            if header_level == 1 {
                return None
            }

            let mut link = match_str.replace(".", "").replace(" ", "-").to_ascii_lowercase();
            let toc_entry = format!("{indentation}- [{title}](#{link})",
                indentation = "  ".repeat(header_level - 2),
                title = match_str.to_string()
            );

            Some(toc_entry)
        }).collect();

        self.output_buffer = self.output_buffer.replace(TOC_TAG, &toc_items.join("\n"));
    }

    /// Run generator
    pub async fn run(&mut self) -> Result<(), GeneratorError> {
        if self.lucid_client.is_some() {
            self.ensure_lucid_token_validity().await?;

            self.write_images_md().await;
        } else {
            // Remove the lucid tag
            self.output_buffer = self.output_buffer.replace(LUCID_TAG, "");
        }

        self.write_cards().await;
        
        self.write_table_of_contents();
        
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
