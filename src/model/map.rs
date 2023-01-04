use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{Placable, Vector3};

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub id: String,
    pub version: String,
    pub name: String,
    pub tiles: BTreeMap<Vector3, Placable>,
    pub objects: BTreeMap<Vector3, Placable>,
}
