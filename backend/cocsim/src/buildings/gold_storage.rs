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

struct GoldStorageLevel {
    pub health: f32,
}

const GOLD_STORAGE_LEVELS: &[GoldStorageLevel] = &[
    GoldStorageLevel { health: 400.0 },
    GoldStorageLevel { health: 600.0 },
    GoldStorageLevel { health: 800.0 },
    GoldStorageLevel { health: 1000.0 },
    GoldStorageLevel { health: 1200.0 },
    GoldStorageLevel { health: 1400.0 },
    GoldStorageLevel { health: 1600.0 },
    GoldStorageLevel { health: 1700.0 },
    GoldStorageLevel { health: 1800.0 },
    GoldStorageLevel { health: 1900.0 },
    GoldStorageLevel { health: 2100.0 },
    GoldStorageLevel { health: 2500.0 },
    GoldStorageLevel { health: 2900.0 },
    GoldStorageLevel { health: 3300.0 },
    GoldStorageLevel { health: 3700.0 },
    GoldStorageLevel { health: 3900.0 },
    GoldStorageLevel { health: 4050.0 },
    GoldStorageLevel { health: 4200.0 },
];

const GOLD_STORAGE: &BuildingType = &BuildingType {
    name: "GoldStorage",
    size: Vector2::new(3, 3),
    levels: GOLD_STORAGE_LEVELS.len(),
    options: &[],
};

#[derive(Serialize, Deserialize, Debug)]
pub struct GoldStorageModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for GoldStorageModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        create_passive_building(
            world,
            GOLD_STORAGE_LEVELS
                .get(self.level)
                .context("Level out of range")?
                .health,
            Vector2::new(self.x, self.y),
            GOLD_STORAGE.size,
            None,
        )?;

        Ok(())
    }
}
