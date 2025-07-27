use anyhow::ensure;
use arbitrary::Arbitrary;
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

const DARK_ELIXIR_STORAGE: BuildingType = BuildingType {
    name: "DarkElixirStorage",
    size: Vector2::new(3, 3),
    levels: DARK_ELIXIR_STORAGE_LEVELS.len(),
    options: &[],
};

inventory::submit! {DARK_ELIXIR_STORAGE}

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub struct DarkElixirStorageModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for DarkElixirStorageModel {
    fn r#type(&self) -> &'static BuildingType {
        &DARK_ELIXIR_STORAGE
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(self.x, self.y)
    }

    fn validate(&self) -> anyhow::Result<()> {
        ensure!(self.level < DARK_ELIXIR_STORAGE_LEVELS.len());

        Ok(())
    }

    fn create_building(&self, world: &mut World) {
        create_passive_building(
            world,
            DARK_ELIXIR_STORAGE_LEVELS[self.level].health,
            Vector2::new(self.x, self.y),
            DARK_ELIXIR_STORAGE.size,
        );
    }
}
