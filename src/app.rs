use std::{path::Path, fs::{self, File}, fmt::Display, io::Write};

use crate::{Args, config::Config};

pub enum GeneratorError {
    InvalidOutputDirectory
}

impl Display for GeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneratorError::InvalidOutputDirectory => write!(f, "Accessing or creating specified output directory").unwrap()
        };
        
        Ok(())
    }
}

pub struct App {
    output_dir: String,
    conf: Config
}

impl App {
    pub fn new(conf: Config, output_dir: &str) -> Result<Self, GeneratorError> {
        fs::create_dir(output_dir).map_err(|_| GeneratorError::InvalidOutputDirectory)?;
        
        Ok(App {
            output_dir: output_dir.to_string(),
            conf
        })
    }

    pub async fn run(&self) -> Result<(), GeneratorError> {
        // Check lucid access and refresh if necessary

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
