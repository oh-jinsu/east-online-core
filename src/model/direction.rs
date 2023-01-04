use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Idle,
    Up,
    Right,
    Down,
    Left,
}
