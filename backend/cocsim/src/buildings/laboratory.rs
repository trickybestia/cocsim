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

struct LaboratoryLevel {
    pub health: f32,
}

const LABORATORY_LEVELS: &[LaboratoryLevel] = &[
    LaboratoryLevel { health: 500.0 },
    LaboratoryLevel { health: 550.0 },
    LaboratoryLevel { health: 600.0 },
    LaboratoryLevel { health: 650.0 },
    LaboratoryLevel { health: 700.0 },
    LaboratoryLevel { health: 750.0 },
    LaboratoryLevel { health: 830.0 },
    LaboratoryLevel { health: 950.0 },
    LaboratoryLevel { health: 1070.0 },
    LaboratoryLevel { health: 1140.0 },
    LaboratoryLevel { health: 1210.0 },
    LaboratoryLevel { health: 1280.0 },
    LaboratoryLevel { health: 1350.0 },
    LaboratoryLevel { health: 1400.0 },
];

const LABORATORY: &BuildingType = &BuildingType {
    name: "Laboratory",
    size: Vector2::new(3, 3),
    levels: LABORATORY_LEVELS.len(),
    options: &[],
};

#[derive(Serialize, Deserialize, Debug)]
pub struct LaboratoryModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for LaboratoryModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        create_passive_building(
            world,
            LABORATORY_LEVELS
                .get(self.level)
                .context("Level out of range")?
                .health,
            Vector2::new(self.x, self.y),
            LABORATORY.size,
            None,
        )?;

        Ok(())
    }
}
