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

struct XBowLevel {
    pub health: f32,
    pub attack_damage: f32,
}

const X_BOW_LEVELS: &[XBowLevel] = &[
    XBowLevel {
        health: 1500.0,
        attack_damage: 7.68,
    },
    XBowLevel {
        health: 1900.0,
        attack_damage: 8.96,
    },
    XBowLevel {
        health: 2300.0,
        attack_damage: 10.24,
    },
    XBowLevel {
        health: 2700.0,
        attack_damage: 10.88,
    },
    XBowLevel {
        health: 3100.0,
        attack_damage: 12.16,
    },
    XBowLevel {
        health: 3400.0,
        attack_damage: 14.08,
    },
    XBowLevel {
        health: 3700.0,
        attack_damage: 16.64,
    },
    XBowLevel {
        health: 4000.0,
        attack_damage: 19.84,
    },
    XBowLevel {
        health: 4200.0,
        attack_damage: 23.68,
    },
    XBowLevel {
        health: 4400.0,
        attack_damage: 26.24,
    },
    XBowLevel {
        health: 4600.0,
        attack_damage: 28.8,
    },
    XBowLevel {
        health: 4800.0,
        attack_damage: 30.08,
    },
];

const X_BOW: BuildingType = BuildingType {
    name: "XBow",
    size: Vector2::new(3, 3),
    levels: X_BOW_LEVELS.len(),
    options: &[BuildingOption {
        name: "target",
        values: &["Ground", "AirAndGround"],
    }],
};

inventory::submit! {X_BOW}

#[derive(Serialize, Deserialize, Debug)]
pub enum XBowTargetType {
    Ground,
    AirAndGround,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XBowModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
    pub target: XBowTargetType,
}

impl BuildingModel for XBowModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        create_passive_building(
            world,
            X_BOW_LEVELS
                .get(self.level)
                .context("Level out of range")?
                .health,
            Vector2::new(self.x, self.y),
            X_BOW.size,
            None,
        )?;

        Ok(())
    }
}
