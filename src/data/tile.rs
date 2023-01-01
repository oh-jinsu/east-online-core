use serde::{Deserialize, Serialize};

use super::TileTexture;

#[derive(Debug, Serialize, Deserialize)]
pub enum Tile {
    Simple { id: String, texture: TileTexture },
    Complex { id: String, model: String },
}
