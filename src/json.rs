use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DefaultInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    artist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    album: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Song {
    // We will use DefaultInfo if these are missing
    #[serde(skip_serializing_if = "Option::is_none")]
    artist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    album: Option<String>,
    title: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Game {
    pub system: String,
    pub hashes: Vec<String>,
    pub names: Vec<String>,
    pub default_info: DefaultInfo,
    pub addr: String,
    pub songs: HashMap<String, Song>,
}

