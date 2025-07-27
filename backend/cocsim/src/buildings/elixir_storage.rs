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

struct ElixirStorageLevel {
    pub health: f32,
}

const ELIXIR_STORAGE_LEVELS: &[ElixirStorageLevel] = &[
    ElixirStorageLevel { health: 400.0 },
    ElixirStorageLevel { health: 600.0 },
    ElixirStorageLevel { health: 800.0 },
    ElixirStorageLevel { health: 1000.0 },
    ElixirStorageLevel { health: 1200.0 },
    ElixirStorageLevel { health: 1400.0 },
    ElixirStorageLevel { health: 1600.0 },
    ElixirStorageLevel { health: 1700.0 },
    ElixirStorageLevel { health: 1800.0 },
    ElixirStorageLevel { health: 1900.0 },
    ElixirStorageLevel { health: 2100.0 },
    ElixirStorageLevel { health: 2500.0 },
    ElixirStorageLevel { health: 2900.0 },
    ElixirStorageLevel { health: 3300.0 },
    ElixirStorageLevel { health: 3700.0 },
    ElixirStorageLevel { health: 3900.0 },
    ElixirStorageLevel { health: 4050.0 },
    ElixirStorageLevel { health: 4200.0 },
];

const ELIXIR_STORAGE: BuildingType = BuildingType {
    name: "ElixirStorage",
    size: Vector2::new(3, 3),
    levels: ELIXIR_STORAGE_LEVELS.len(),
    options: &[],
};

inventory::submit! {ELIXIR_STORAGE}

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub struct ElixirStorageModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for ElixirStorageModel {
    fn r#type(&self) -> &'static BuildingType {
        &ELIXIR_STORAGE
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(self.x, self.y)
    }

    fn validate(&self) -> anyhow::Result<()> {
        ensure!(self.level < ELIXIR_STORAGE_LEVELS.len());

        Ok(())
    }

    fn create_building(&self, world: &mut World) {
        create_passive_building(
            world,
            ELIXIR_STORAGE_LEVELS[self.level].health,
            Vector2::new(self.x, self.y),
            ELIXIR_STORAGE.size,
            None,
        );
    }
}
