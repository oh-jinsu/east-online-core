use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MapManifest {
    pub items: Vec<MapManifestItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapManifestItem {
    pub id: String,
    pub name: String,
    pub version: String,
}
