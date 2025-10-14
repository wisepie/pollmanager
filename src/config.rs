use dirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, BufReader, Error, ErrorKind, Result},
    path::{Path, PathBuf},
};

const FILE_NAME: &str = "config.json";
const APP_NAME: &str = "dinkdonk-poll-manager";

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub apikey: String,
    pub path: Option<PathBuf>,
}
impl Config {
    pub fn get_or_build_path(&mut self) -> Result<()> {
        match dirs::config_dir() {
            Some(config_path) => {
                let path = Path::new(&config_path);
                let app_config_path = path.join(APP_NAME);

                if !app_config_path.exists() {
                    fs::create_dir(&app_config_path)?;
                }

                let config_file_path = &app_config_path.join(FILE_NAME);

                self.path = Some(config_file_path.to_path_buf());

                Ok(())
            }
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No config directory found",
            )),
        }
    }
    pub fn check_key(&mut self, apikey_input: String) -> Result<()> {
        if apikey_input.len() == 32 {
            self.apikey = apikey_input;
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "API Key Format is Incorrect",
            ))
        }
    }
    pub fn read_file(&mut self) -> Result<()> {
        let file = File::open(self.path.as_ref().unwrap());

        match file.is_err() {
            true => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Config file not found",
            )),
            false => {
                let reader = BufReader::new(file.unwrap());
                let config: Config = serde_json::from_reader(reader)?;
                *self = config;
                Ok(())
            }
        }
    }
    pub fn save_file(&mut self) -> Result<()> {
        std::fs::write(
            self.path.as_ref().unwrap(),
            serde_json::to_string_pretty(&self).unwrap(),
        )?;
        Ok(())
    }
}
impl Default for Config {
    fn default() -> Self {
        Config {
            apikey: "".to_owned(),
            path: None,
        }
    }
}
