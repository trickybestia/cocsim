use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};
use shipyard::World;

use crate::{
    BuildingModel,
    BuildingType,
};

struct ArmyCampLevel {
    pub health: f32,
}

const ARMY_CAMP_LEVELS: &[ArmyCampLevel] = &[
    ArmyCampLevel { health: 250.0 },
    ArmyCampLevel { health: 270.0 },
    ArmyCampLevel { health: 290.0 },
    ArmyCampLevel { health: 310.0 },
    ArmyCampLevel { health: 330.0 },
    ArmyCampLevel { health: 350.0 },
    ArmyCampLevel { health: 400.0 },
    ArmyCampLevel { health: 500.0 },
    ArmyCampLevel { health: 600.0 },
    ArmyCampLevel { health: 700.0 },
    ArmyCampLevel { health: 800.0 },
    ArmyCampLevel { health: 850.0 },
    ArmyCampLevel { health: 900.0 },
];

const ARMY_CAMP: &BuildingType = &BuildingType {
    name: "ArmyCamp",
    size: Vector2::new(4, 4),
    levels: ARMY_CAMP_LEVELS.len(),
    options: vec![],
};

#[derive(Serialize, Deserialize, Debug)]
pub struct ArmyCampModel {
    x: u32,
    y: u32,
    level: u32,
}

impl BuildingModel for ArmyCampModel {
    fn create_building(&self, world: &mut World) {}
}
