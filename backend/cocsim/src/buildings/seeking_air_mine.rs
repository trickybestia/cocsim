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
    buildings::utils::trap::spawn_trap,
    consts::MAX_BUILDING_POS,
};

struct SeekingAirMineLevel {
    pub damage: f32,
}

const SEEKING_AIR_MINE_LEVELS_LEN: usize = 7;
const SEEKING_AIR_MINE_LEVEL_INDEX_MAX: usize = SEEKING_AIR_MINE_LEVELS_LEN - 1;
const SEEKING_AIR_MINE_LEVELS: [SeekingAirMineLevel; SEEKING_AIR_MINE_LEVELS_LEN] = [
    SeekingAirMineLevel { damage: 1500.0 },
    SeekingAirMineLevel { damage: 1800.0 },
    SeekingAirMineLevel { damage: 2100.0 },
    SeekingAirMineLevel { damage: 2500.0 },
    SeekingAirMineLevel { damage: 2800.0 },
    SeekingAirMineLevel { damage: 3000.0 },
    SeekingAirMineLevel { damage: 3200.0 },
];

const SEEKING_AIR_MINE: BuildingType = BuildingType {
    name: "SeekingAirMine",
    size: Vector2::new(1, 1),
    levels: SEEKING_AIR_MINE_LEVELS.len(),
    options: &[],
};

inventory::submit! {SEEKING_AIR_MINE}

const SEEKING_AIR_MINE_TRIGGER_RADIUS: f32 = 4.0;
const SEEKING_AIR_MINE_SPEED: f32 = 3.5;

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct SeekingAirMineModel {
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<SEEKING_AIR_MINE_LEVEL_INDEX_MAX>,
}

impl BuildingModel for SeekingAirMineModel {
    fn r#type(&self) -> &'static BuildingType {
        &SEEKING_AIR_MINE
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn spawn(&self, world: &mut World) {
        spawn_trap(world, self.position(), SEEKING_AIR_MINE.size);
    }
}
