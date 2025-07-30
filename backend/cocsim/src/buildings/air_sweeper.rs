use arbitrary::Arbitrary;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};
use shipyard::World;

use crate::{
    BuildingModel,
    BuildingOption,
    BuildingType,
    UsizeWithMax,
    buildings::utils::passive_building::create_passive_building,
    consts::MAX_BUILDING_POS,
};

struct AirSweeperLevel {
    pub health: f32,
    pub push_strength: f32,
}

const AIR_SWEEPER_LEVELS_LEN: usize = 7;
const AIR_SWEEPER_LEVEL_INDEX_MAX: usize = AIR_SWEEPER_LEVELS_LEN - 1;
const AIR_SWEEPER_LEVELS: [AirSweeperLevel; AIR_SWEEPER_LEVELS_LEN] = [
    AirSweeperLevel {
        health: 750.0,
        push_strength: 1.6,
    },
    AirSweeperLevel {
        health: 800.0,
        push_strength: 2.0,
    },
    AirSweeperLevel {
        health: 850.0,
        push_strength: 2.4,
    },
    AirSweeperLevel {
        health: 900.0,
        push_strength: 2.8,
    },
    AirSweeperLevel {
        health: 950.0,
        push_strength: 3.2,
    },
    AirSweeperLevel {
        health: 1000.0,
        push_strength: 3.6,
    },
    AirSweeperLevel {
        health: 1050.0,
        push_strength: 4.0,
    },
];

const AIR_SWEEPER: BuildingType = BuildingType {
    name: "AirSweeper",
    size: Vector2::new(2, 2),
    levels: AIR_SWEEPER_LEVELS.len(),
    options: &[BuildingOption {
        name: "rotation",
        values: &[
            "Right",
            "RightUp",
            "Up",
            "LeftUp",
            "Left",
            "LeftDown",
            "Down",
            "RightDown",
        ],
    }],
};

inventory::submit! {AIR_SWEEPER}

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub enum AirSweeperRotation {
    Right,
    RightUp,
    Up,
    LeftUp,
    Left,
    LeftDown,
    Down,
    RightDown,
}

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub struct AirSweeperModel {
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<AIR_SWEEPER_LEVEL_INDEX_MAX>,
    pub rotation: AirSweeperRotation,
}

impl BuildingModel for AirSweeperModel {
    fn r#type(&self) -> &'static BuildingType {
        &AIR_SWEEPER
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn create_building(&self, world: &mut World) {
        create_passive_building(
            world,
            AIR_SWEEPER_LEVELS[*self.level].health,
            Vector2::new(*self.x, *self.y),
            AIR_SWEEPER.size,
        );
    }
}
