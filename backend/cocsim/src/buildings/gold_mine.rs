use arbitrary::Arbitrary;
use hecs::World;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    UsizeWithMax,
    buildings::{
        BuildingModel,
        BuildingType,
        utils::resource_building::spawn_resource_building,
    },
};
struct GoldMineLevel {
    pub health: f32,
}

const GOLD_MINE_LEVELS_LEN: usize = 16;
const GOLD_MINE_LEVEL_INDEX_MAX: usize = GOLD_MINE_LEVELS_LEN - 1;
const GOLD_MINE_LEVELS: [GoldMineLevel; GOLD_MINE_LEVELS_LEN] = [
    GoldMineLevel { health: 400.0 },
    GoldMineLevel { health: 440.0 },
    GoldMineLevel { health: 480.0 },
    GoldMineLevel { health: 520.0 },
    GoldMineLevel { health: 560.0 },
    GoldMineLevel { health: 600.0 },
    GoldMineLevel { health: 640.0 },
    GoldMineLevel { health: 680.0 },
    GoldMineLevel { health: 720.0 },
    GoldMineLevel { health: 780.0 },
    GoldMineLevel { health: 860.0 },
    GoldMineLevel { health: 960.0 },
    GoldMineLevel { health: 1080.0 },
    GoldMineLevel { health: 1180.0 },
    GoldMineLevel { health: 1280.0 },
    GoldMineLevel { health: 1350.0 },
];

const GOLD_MINE: BuildingType = BuildingType {
    name: "GoldMine",
    size: Vector2::new(3, 3),
    levels: GOLD_MINE_LEVELS.len(),
    options: &[],
    affects_drop_zone: true,
};

inventory::submit! {GOLD_MINE}

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct GoldMineModel {
    pub level: UsizeWithMax<GOLD_MINE_LEVEL_INDEX_MAX>,
}

impl BuildingModel for GoldMineModel {
    fn r#type(&self) -> &'static BuildingType {
        &GOLD_MINE
    }

    fn spawn(&self, world: &mut World, position: Vector2<usize>) {
        spawn_resource_building(
            world,
            GOLD_MINE_LEVELS[*self.level].health,
            position,
            GOLD_MINE.size,
        );
    }
}
