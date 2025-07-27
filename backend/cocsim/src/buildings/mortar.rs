use anyhow::{
    Context,
    Result,
};
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

struct MortarLevel {
    pub health: f32,
    pub attack_damage: f32,
}

const MORTAR_LEVELS: &[MortarLevel] = &[
    MortarLevel {
        health: 400.0,
        attack_damage: 20.0,
    },
    MortarLevel {
        health: 450.0,
        attack_damage: 25.0,
    },
    MortarLevel {
        health: 500.0,
        attack_damage: 30.0,
    },
    MortarLevel {
        health: 550.0,
        attack_damage: 35.0,
    },
    MortarLevel {
        health: 600.0,
        attack_damage: 45.0,
    },
    MortarLevel {
        health: 650.0,
        attack_damage: 55.0,
    },
    MortarLevel {
        health: 700.0,
        attack_damage: 75.0,
    },
    MortarLevel {
        health: 800.0,
        attack_damage: 100.0,
    },
    MortarLevel {
        health: 950.0,
        attack_damage: 125.0,
    },
    MortarLevel {
        health: 1100.0,
        attack_damage: 150.0,
    },
    MortarLevel {
        health: 1300.0,
        attack_damage: 175.0,
    },
    MortarLevel {
        health: 1500.0,
        attack_damage: 190.0,
    },
    MortarLevel {
        health: 1700.0,
        attack_damage: 210.0,
    },
    MortarLevel {
        health: 1950.0,
        attack_damage: 240.0,
    },
    MortarLevel {
        health: 2150.0,
        attack_damage: 270.0,
    },
    MortarLevel {
        health: 2300.0,
        attack_damage: 300.0,
    },
    MortarLevel {
        health: 2450.0,
        attack_damage: 330.0,
    },
];

const MORTAR: BuildingType = BuildingType {
    name: "Mortar",
    size: Vector2::new(3, 3),
    levels: MORTAR_LEVELS.len(),
    options: &[],
};

inventory::submit! {MORTAR}

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub struct MortarModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for MortarModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        create_passive_building(
            world,
            MORTAR_LEVELS
                .get(self.level)
                .context("Level out of range")?
                .health,
            Vector2::new(self.x, self.y),
            MORTAR.size,
            None,
        )?;

        Ok(())
    }
}
