use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Direction {
    Idle,
    Up,
    Down,
    Left,
    Right,
}
