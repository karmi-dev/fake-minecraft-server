use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub version: Version,

    /// Optional fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub players: Option<Players>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "description"))]
    pub motd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<Vec<Player>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub id: String,
}
