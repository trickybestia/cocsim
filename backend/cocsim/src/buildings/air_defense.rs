use anyhow::ensure;
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
    buildings::utils::active_building::create_active_building,
    game::features::attack::{
        BuildingFindTargetBehaviour,
        TargetProjectileAttackBehaviour,
    },
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

const AIR_DEFENSE_ATTACK_RANGE: f32 = 10.0;
const AIR_DEFENSE_ATTACK_COOLDOWN: f32 = 1.0;
const AIR_DEFENSE_PROJECTILE_SPEED: f32 = 8.0;

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub struct AirDefenseModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for AirDefenseModel {
    fn r#type(&self) -> &'static BuildingType {
        &AIR_DEFENSE
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(self.x, self.y)
    }

    fn validate(&self) -> anyhow::Result<()> {
        ensure!(self.level < AIR_DEFENSE_LEVELS.len());

        Ok(())
    }

    fn create_building(&self, world: &mut World) {
        let level = &AIR_DEFENSE_LEVELS[self.level];

        create_active_building(
            world,
            level.health,
            Vector2::new(self.x, self.y),
            AIR_DEFENSE.size,
            AIR_DEFENSE_ATTACK_RANGE,
            AIR_DEFENSE_ATTACK_COOLDOWN,
            BuildingFindTargetBehaviour {
                attack_air: true,
                attack_ground: false,
            }
            .into(),
            TargetProjectileAttackBehaviour {
                damage: level.attack_damage,
                projectile_speed: AIR_DEFENSE_PROJECTILE_SPEED,
            }
            .into(),
        );
    }
}
