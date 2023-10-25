use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::APP_NAME;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
  pub id: String,
  pub  host: String
}

fn default_config() -> Config {
    let my_uuid = Uuid::new_v4();
    Config {
        id: my_uuid.to_string(),
        host: "TBA".to_string(),
    }
}



pub fn read_config() -> Option<Config> {
    // Determine the config directory based on the OS
    let mut config_path: PathBuf = match dirs::config_dir() {
        Some(path) => path,
        None => return None,
    };

    config_path.push(APP_NAME);
    config_path.push("config.toml");


// Create the config file with default values if it doesn't exist
    if !config_path.exists() {
        let default_config = default_config();
        let default_contents = toml::to_string(&default_config).unwrap();
        
        let mut file = match OpenOptions::new().create(true).write(true).open(&config_path) {
            Ok(file) => file,
            Err(_) => return None,
        };
        
        if file.write_all(default_contents.as_bytes()).is_err() {
            return None;
        }
    }

    // Read the file
    let mut file = match File::open(&config_path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return None;
    }

    // Parse TOML
    match toml::from_str::<Config>(&contents) {
        Ok(config) => Some(config),
        Err(_) => None,
    }
}


