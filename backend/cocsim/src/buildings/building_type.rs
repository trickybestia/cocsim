use std::ops::Deref;

use nalgebra::Vector2;
use serde::Serialize;

use crate::{
    BuildingOption,
    EntityBehaviour,
};

#[derive(Serialize)]
pub struct BuildingType {
    name: String,
    size: Vector2<u32>,
    levels: u32,
    options: Vec<BuildingOption>,

    #[serde(skip)]
    behaviour: Box<dyn EntityBehaviour>,
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

impl Deref for BuildingType {
    type Target = Box<dyn EntityBehaviour>;

    fn deref(&self) -> &Self::Target {
        &self.behaviour
    }
}
