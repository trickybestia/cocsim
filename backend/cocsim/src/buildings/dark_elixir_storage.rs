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

struct DarkElixirStorageLevel {
    pub health: f32,
}

const DARK_ELIXIR_STORAGE_LEVELS: &[DarkElixirStorageLevel] = &[
    DarkElixirStorageLevel { health: 2000.0 },
    DarkElixirStorageLevel { health: 2200.0 },
    DarkElixirStorageLevel { health: 2400.0 },
    DarkElixirStorageLevel { health: 2600.0 },
    DarkElixirStorageLevel { health: 2900.0 },
    DarkElixirStorageLevel { health: 3200.0 },
    DarkElixirStorageLevel { health: 3500.0 },
    DarkElixirStorageLevel { health: 3800.0 },
    DarkElixirStorageLevel { health: 4100.0 },
    DarkElixirStorageLevel { health: 4300.0 },
    DarkElixirStorageLevel { health: 4500.0 },
    DarkElixirStorageLevel { health: 4700.0 },
];

const DARK_ELIXIR_STORAGE: &BuildingType = &BuildingType {
    name: "DarkElixirStorage",
    size: Vector2::new(3, 3),
    levels: DARK_ELIXIR_STORAGE_LEVELS.len(),
    options: &[],
};

#[derive(Serialize, Deserialize, Debug)]
pub struct DarkElixirStorageModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for DarkElixirStorageModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        create_passive_building(
            world,
            DARK_ELIXIR_STORAGE_LEVELS
                .get(self.level)
                .context("Level out of range")?
                .health,
            Vector2::new(self.x, self.y),
            DARK_ELIXIR_STORAGE.size,
            None,
        )?;

        Ok(())
    }
}
