use base64::{engine::general_purpose as base64_engine, Engine as _};
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
                description: Some("§cFake Minecraft Server".to_string()),
                favicon: Some("server-icon.png".to_string()),
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
        let mut config: Config = serde_yaml::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // Handle the favicon
        if let Some(favicon) = config.status.favicon {
            match Self::load_favicon_as_base64(favicon) {
                Ok(favicon_base64) => config.status.favicon = Some(favicon_base64),
                Err(e) => {
                    eprintln!("Error loading favicon: {}", e);
                    config.status.favicon = None;
                }
            }
        }

        Ok(config)
    }

    fn load_favicon_as_base64(favicon: String) -> Result<String, io::Error> {
        if favicon.starts_with("data:image/png;base64,") {
            Ok(favicon)
        } else {
            let favicon_path = Path::new(&favicon);
            if favicon_path.extension().and_then(|ext| ext.to_str()) != Some("png") {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Favicon must be a PNG file",
                ));
            }

            if !favicon_path.exists() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Favicon file doesn't exist: {}", favicon_path.display()),
                ));
            }

            let favicon_bytes = fs::read(favicon)?;
            let favicon_base64 = base64_engine::STANDARD.encode(&favicon_bytes);

            Ok(format!("data:image/png;base64,{}", favicon_base64))
        }
    }
}
