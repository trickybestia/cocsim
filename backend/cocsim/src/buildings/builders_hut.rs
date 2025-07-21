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

struct BuildersHutLevel {
    pub health: f32,
}

const BUILDERS_HUT_LEVELS: &[BuildersHutLevel] = &[
    BuildersHutLevel { health: 250.0 },
    BuildersHutLevel { health: 1000.0 },
    BuildersHutLevel { health: 1300.0 },
    BuildersHutLevel { health: 1600.0 },
    BuildersHutLevel { health: 1800.0 },
    BuildersHutLevel { health: 1900.0 },
    BuildersHutLevel { health: 2000.0 },
];

const BUILDERS_HUT: &BuildingType = &BuildingType {
    name: "BuildersHut",
    size: Vector2::new(2, 2),
    levels: BUILDERS_HUT_LEVELS.len(),
    options: &[],
};

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildersHutModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for BuildersHutModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        create_passive_building(
            world,
            BUILDERS_HUT_LEVELS
                .get(self.level)
                .context("Level out of range")?
                .health,
            Vector2::new(self.x, self.y),
            BUILDERS_HUT.size,
            None,
        )?;

        Ok(())
    }
}
