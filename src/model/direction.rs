use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Idle,
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn to_bytes(&self) -> [u8; 1] {
        match self {
            Direction::Idle => [0],
            Direction::Up => [1],
            Direction::Right => [2],
            Direction::Down => [3],
            Direction::Left => [4],
        }
    }
}
