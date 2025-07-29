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
    game::features::actions::{
        BuildingFindTarget,
        TargetProjectileAttack,
    },
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

const CANNON_MIN_ATTACK_RANGE: f32 = 0.0;
const CANNON_MAX_ATTACK_RANGE: f32 = 9.0;
const CANNON_ATTACK_COOLDOWN: f32 = 0.8;
const CANNON_PROJECTILE_SPEED: f32 = 12.0;

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub struct CannonModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for CannonModel {
    fn r#type(&self) -> &'static BuildingType {
        &CANNON
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(self.x, self.y)
    }

    fn validate(&self) -> anyhow::Result<()> {
        ensure!(self.level < CANNON_LEVELS.len());

        Ok(())
    }

    fn create_building(&self, world: &mut World) {
        let level = &CANNON_LEVELS[self.level];

        create_active_building(
            world,
            level.health,
            Vector2::new(self.x, self.y),
            CANNON.size,
            CANNON_MIN_ATTACK_RANGE,
            CANNON_MAX_ATTACK_RANGE,
            CANNON_ATTACK_COOLDOWN,
            BuildingFindTarget {
                attack_air: false,
                attack_ground: true,
            }
            .into(),
            TargetProjectileAttack {
                damage: level.attack_damage,
                projectile_speed: CANNON_PROJECTILE_SPEED,
            }
            .into(),
        );
    }
}
