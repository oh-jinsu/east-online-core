use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation {
    Up,
    Right,
    Down,
    Left,
}

impl Rotation {
    pub fn to_bytes(&self) -> [u8; 1] {
        match self {
            Rotation::Up => [0],
            Rotation::Right => [1],
            Rotation::Down => [2],
            Rotation::Left => [3],
        }
    }
}
