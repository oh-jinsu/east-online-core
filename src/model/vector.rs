use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Ord, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl PartialOrd for Vector3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.y.partial_cmp(&other.y) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.z.partial_cmp(&other.z) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.x.partial_cmp(&other.x)
    }
}
