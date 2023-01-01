use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TileTexture {
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32,
    pub front: i32,
    pub back: i32,
}
