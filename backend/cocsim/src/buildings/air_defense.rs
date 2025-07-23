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

struct AirDefenseLevel {
    pub health: f32,
    pub attack_damage: f32,
}

const AIR_DEFENSE_LEVELS: &[AirDefenseLevel] = &[
    AirDefenseLevel {
        health: 800.0,
        attack_damage: 80.0,
    },
    AirDefenseLevel {
        health: 850.0,
        attack_damage: 110.0,
    },
    AirDefenseLevel {
        health: 900.0,
        attack_damage: 140.0,
    },
    AirDefenseLevel {
        health: 950.0,
        attack_damage: 160.0,
    },
    AirDefenseLevel {
        health: 1000.0,
        attack_damage: 190.0,
    },
    AirDefenseLevel {
        health: 1050.0,
        attack_damage: 230.0,
    },
    AirDefenseLevel {
        health: 1100.0,
        attack_damage: 280.0,
    },
    AirDefenseLevel {
        health: 1210.0,
        attack_damage: 320.0,
    },
    AirDefenseLevel {
        health: 1300.0,
        attack_damage: 360.0,
    },
    AirDefenseLevel {
        health: 1400.0,
        attack_damage: 400.0,
    },
    AirDefenseLevel {
        health: 1500.0,
        attack_damage: 440.0,
    },
    AirDefenseLevel {
        health: 1650.0,
        attack_damage: 500.0,
    },
    AirDefenseLevel {
        health: 1750.0,
        attack_damage: 540.0,
    },
    AirDefenseLevel {
        health: 1850.0,
        attack_damage: 600.0,
    },
    AirDefenseLevel {
        health: 1950.0,
        attack_damage: 650.0,
    },
];

const AIR_DEFENSE: BuildingType = BuildingType {
    name: "AirDefense",
    size: Vector2::new(3, 3),
    levels: AIR_DEFENSE_LEVELS.len(),
    options: &[],
};

inventory::submit! {AIR_DEFENSE}

#[derive(Serialize, Deserialize, Debug)]
pub struct AirDefenseModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for AirDefenseModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        create_passive_building(
            world,
            AIR_DEFENSE_LEVELS
                .get(self.level)
                .context("Level out of range")?
                .health,
            Vector2::new(self.x, self.y),
            AIR_DEFENSE.size,
            None,
        )?;

        Ok(())
    }
}
