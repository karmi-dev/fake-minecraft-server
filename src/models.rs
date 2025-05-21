use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub version: Version,
    pub players: Option<Players>,
    pub description: String,
    pub favicon: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Version {
    #[serde(flatten)]
    pub info: Option<VersionInfo>,

    #[serde(skip_serializing)]
    pub same: Option<bool>,
}
impl Default for Version {
    fn default() -> Self {
        Version {
            info: None,
            same: Some(true),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub name: String,
    pub protocol: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    pub sample: Option<Vec<Player>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub id: String,
}
