use arbitrary::Arbitrary;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};
use hecs::World;

use crate::{
    BuildingModel,
    BuildingType,
    UsizeWithMax,
    buildings::utils::passive_building::create_passive_building,
    consts::MAX_BUILDING_POS,
};

struct GoldStorageLevel {
    pub health: f32,
}

const GOLD_STORAGE_LEVELS_LEN: usize = 18;
const GOLD_STORAGE_LEVEL_INDEX_MAX: usize = GOLD_STORAGE_LEVELS_LEN - 1;
const GOLD_STORAGE_LEVELS: [GoldStorageLevel; GOLD_STORAGE_LEVELS_LEN] = [
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

const GOLD_STORAGE: BuildingType = BuildingType {
    name: "GoldStorage",
    size: Vector2::new(3, 3),
    levels: GOLD_STORAGE_LEVELS.len(),
    options: &[],
};

inventory::submit! {GOLD_STORAGE}

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct GoldStorageModel {
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<GOLD_STORAGE_LEVEL_INDEX_MAX>,
}

impl BuildingModel for GoldStorageModel {
    fn r#type(&self) -> &'static BuildingType {
        &GOLD_STORAGE
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn spawn(&self, world: &mut World) {
        create_passive_building(
            world,
            GOLD_STORAGE_LEVELS[*self.level].health,
            Vector2::new(*self.x, *self.y),
            GOLD_STORAGE.size,
        );
    }
}
