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
    BuildingOption,
    BuildingType,
    buildings::utils::passive_building::create_passive_building,
};

struct AirSweeperLevel {
    pub health: f32,
    pub push_strength: f32,
}

const AIR_SWEEPER_LEVELS: &[AirSweeperLevel] = &[
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct AirSweeperModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
    pub rotation: AirSweeperRotation,
}

impl BuildingModel for AirSweeperModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        create_passive_building(
            world,
            AIR_SWEEPER_LEVELS
                .get(self.level)
                .context("Level out of range")?
                .health,
            Vector2::new(self.x, self.y),
            AIR_SWEEPER.size,
            None,
        )?;

        Ok(())
    }
}
