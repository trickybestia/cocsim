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

struct CannonLevel {
    pub health: f32,
    pub attack_damage: f32,
}

const CANNON_LEVELS: &[CannonLevel] = &[
    CannonLevel {
        health: 420.0,
        attack_damage: 7.2,
    },
    CannonLevel {
        health: 470.0,
        attack_damage: 8.8,
    },
    CannonLevel {
        health: 520.0,
        attack_damage: 12.0,
    },
    CannonLevel {
        health: 570.0,
        attack_damage: 15.2,
    },
    CannonLevel {
        health: 620.0,
        attack_damage: 20.0,
    },
    CannonLevel {
        health: 670.0,
        attack_damage: 24.8,
    },
    CannonLevel {
        health: 730.0,
        attack_damage: 32.0,
    },
    CannonLevel {
        health: 800.0,
        attack_damage: 38.4,
    },
    CannonLevel {
        health: 880.0,
        attack_damage: 44.8,
    },
    CannonLevel {
        health: 960.0,
        attack_damage: 51.2,
    },
    CannonLevel {
        health: 1060.0,
        attack_damage: 59.2,
    },
    CannonLevel {
        health: 1160.0,
        attack_damage: 68.0,
    },
    CannonLevel {
        health: 1260.0,
        attack_damage: 76.0,
    },
    CannonLevel {
        health: 1380.0,
        attack_damage: 80.0,
    },
    CannonLevel {
        health: 1500.0,
        attack_damage: 84.0,
    },
    CannonLevel {
        health: 1620.0,
        attack_damage: 88.0,
    },
    CannonLevel {
        health: 1740.0,
        attack_damage: 92.0,
    },
    CannonLevel {
        health: 1870.0,
        attack_damage: 100.0,
    },
    CannonLevel {
        health: 2000.0,
        attack_damage: 108.0,
    },
    CannonLevel {
        health: 2150.0,
        attack_damage: 120.0,
    },
    CannonLevel {
        health: 2250.0,
        attack_damage: 128.0,
    },
];

const CANNON: BuildingType = BuildingType {
    name: "Cannon",
    size: Vector2::new(3, 3),
    levels: CANNON_LEVELS.len(),
    options: &[],
};

inventory::submit! {CANNON}

#[derive(Serialize, Deserialize, Debug)]
pub struct CannonModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for CannonModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        create_passive_building(
            world,
            CANNON_LEVELS
                .get(self.level)
                .context("Level out of range")?
                .health,
            Vector2::new(self.x, self.y),
            CANNON.size,
            None,
        )?;

        Ok(())
    }
}
