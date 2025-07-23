use anyhow::{
    Context,
    Result,
};
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};
use shipyard::World;

use crate::{
    BuildingModel,
    BuildingType,
    buildings::utils::passive_building::create_passive_building,
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

const ARMY_CAMP: BuildingType = BuildingType {
    name: "ArmyCamp",
    size: Vector2::new(4, 4),
    levels: ARMY_CAMP_LEVELS.len(),
    options: &[],
};

inventory::submit! {ARMY_CAMP}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArmyCampModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for ArmyCampModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        create_passive_building(
            world,
            ARMY_CAMP_LEVELS
                .get(self.level)
                .context("Level out of range")?
                .health,
            Vector2::new(self.x, self.y),
            ARMY_CAMP.size,
            None,
        )?;

        Ok(())
    }
}
