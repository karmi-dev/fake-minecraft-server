use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

use crate::models::{Players, StatusResponse, Version};

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub status: StatusResponse,
    #[serde(rename = "kick_message")]
    pub kick_msg: String,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            host: "127.0.0.1".to_string(),
            port: 25565,
            status: StatusResponse {
                version: Version::default(),
                players: Some(Players {
                    max: 20,
                    online: 0,
                    sample: None,
                }),
                description: "§cFake Minecraft Server".to_string(),
                favicon: None,
            },
            kick_msg: "§c§lThis is a fake server!\n§eIt only responds to ping requests."
                .to_string(),
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        if !path.as_ref().exists() {
            println!("Config file not found, using default configuration.");
            return Ok(Config::default());
        }

        let contents = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(config)
    }
}
