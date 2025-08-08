use arbitrary::Arbitrary;
use hecs::World;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    BuildingModel,
    BuildingType,
    UsizeWithMax,
    buildings::utils::resource_building::spawn_resource_building,
    consts::MAX_BUILDING_POS,
};

struct DarkElixirStorageLevel {
    pub health: f32,
}

const DARK_ELIXIR_STORAGE_LEVELS_LEN: usize = 12;
const DARK_ELIXIR_STORAGE_LEVEL_INDEX_MAX: usize = DARK_ELIXIR_STORAGE_LEVELS_LEN - 1;
const DARK_ELIXIR_STORAGE_LEVELS: [DarkElixirStorageLevel; DARK_ELIXIR_STORAGE_LEVELS_LEN] = [
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

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct DarkElixirStorageModel {
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<DARK_ELIXIR_STORAGE_LEVEL_INDEX_MAX>,
}

impl BuildingModel for DarkElixirStorageModel {
    fn r#type(&self) -> &'static BuildingType {
        &DARK_ELIXIR_STORAGE
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn spawn(&self, world: &mut World) {
        spawn_resource_building(
            world,
            DARK_ELIXIR_STORAGE_LEVELS[*self.level].health,
            Vector2::new(*self.x, *self.y),
            DARK_ELIXIR_STORAGE.size,
        );
    }
}
