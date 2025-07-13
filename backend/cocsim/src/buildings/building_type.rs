use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};

use crate::BuildingOption;

#[derive(Serialize, Deserialize)]
pub struct BuildingType {
    name: String,
    size: Vector2<u32>,
    levels: u32,
    options: Vec<BuildingOption>,
}

impl BuildingType {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> Vector2<u32> {
        self.size
    }

    pub fn levels(&self) -> u32 {
        self.levels
    }

    pub fn options(&self) -> &[BuildingOption] {
        &self.options
    }
}
