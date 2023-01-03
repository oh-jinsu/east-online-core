use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::models::{Rotation, Vector3};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Placable {
    pub id: String,
    pub rotation: Rotation,
}

impl Placable {
    pub fn repeat(&self, x: i32, y: i32, z: i32) -> BTreeMap<Vector3, Self> {
        let mut result = BTreeMap::default();

        for x in 0..x {
            for z in 0..z {
                for y in 0..y {
                    result.insert(Vector3 { x, y, z }, self.clone());
                }
            }
        }

        result
    }
}
