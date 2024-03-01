use serde::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
}

pub fn read_config() -> Option<Config> {
    let contents = match fs::read_to_string("config.toml") {
        Ok(c) => c,
        Err(e) => {
            println!("Error reading config file: {}", e);
            return None;
        }
    };
    match toml::from_str(&contents) {
        Ok(config) => Some(config),
        Err(e) => {
            println!("Error parsing config file: {}", e);
            None
        }
    }
}
