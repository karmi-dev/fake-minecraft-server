use serde::Serialize;

pub const PROTOCOL_VERSION: i32 = 762; // Minecraft 1.19.4

#[derive(Serialize)]
pub struct StatusResponse {
    pub version: Version,
    pub players: Players,
    pub description: Description,
}

#[derive(Serialize)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Serialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
}

#[derive(Serialize)]
pub struct Description {
    pub text: String,
}
