use base64::{engine::general_purpose as base64_engine, Engine as _};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};
use tracing::{error, info};

use crate::models::{Players, StatusResponse, Version};

fn default_false() -> bool {
    false
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    // This field is used to determine if the config file was found
    #[serde(skip, default = "default_false")]
    default: bool,

    pub debug: bool,
    pub host: String,
    pub port: u16,
    pub status: StatusResponse,
    #[serde(rename = "kick_message")]
    pub kick_msg: String,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            default: true,
            debug: false,
            host: "0.0.0.0".to_string(),
            port: 25565,
            status: StatusResponse {
                version: Version::default(),
                players: Some(Players {
                    max: 20,
                    online: 0,
                    sample: None,
                }),
                motd: Some("§cFake Minecraft Server".to_string()),
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
            return Ok(Config::default());
        }

        let contents = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(config)
    }

    pub fn handle_logs(&self) {
        if self.default {
            info!("Config file not found, using default configuration");
        }
    }

    pub fn handle_favicon(&mut self) {
        if let Some(favicon) = &self.status.favicon {
            match Self::load_favicon_as_base64(favicon.to_string()) {
                Ok(favicon_base64) => self.status.favicon = Some(favicon_base64),
                Err(e) => {
                    // Only log favicon errors for custom configs, since default config includes a known favicon
                    if !self.default {
                        error!("Error loading favicon: {}", e);
                    }

                    self.status.favicon = None;
                }
            }
        }
    }

    fn load_favicon_as_base64(favicon: String) -> Result<String, io::Error> {
        if favicon.starts_with("data:image/png;base64,") {
            return Ok(favicon);
        }

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
