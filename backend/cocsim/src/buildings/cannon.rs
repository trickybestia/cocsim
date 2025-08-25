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
        utils::defensive_building::spawn_defensive_building,
    },
    game::features::{
        actions::TargetProjectileAttack,
        attack::{
            BuildingRetargetCondition,
            targeting::building::BuildingFindTarget,
        },
    },
};

struct CannonLevel {
    pub health: f32,
    pub attack_damage: f32,
}

const CANNON_LEVELS_LEN: usize = 21;
const CANNON_LEVEL_INDEX_MAX: usize = CANNON_LEVELS_LEN - 1;
const CANNON_LEVELS: [CannonLevel; CANNON_LEVELS_LEN] = [
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
    affects_drop_zone: true,
};

inventory::submit! {CANNON}

const CANNON_MIN_ATTACK_RANGE: f32 = 0.0;
const CANNON_MAX_ATTACK_RANGE: f32 = 9.0;
const CANNON_ATTACK_COOLDOWN: f32 = 0.8;
const CANNON_PROJECTILE_SPEED: f32 = 12.0;

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct CannonModel {
    pub level: UsizeWithMax<CANNON_LEVEL_INDEX_MAX>,
}

impl BuildingModel for CannonModel {
    fn r#type(&self) -> &'static BuildingType {
        &CANNON
    }

    fn spawn(&self, world: &mut World, position: Vector2<usize>) {
        let level = &CANNON_LEVELS[*self.level];

        let id = spawn_defensive_building(
            world,
            level.health,
            position,
            CANNON.size,
            CANNON_ATTACK_COOLDOWN,
            BuildingRetargetCondition {
                min_attack_range: CANNON_MIN_ATTACK_RANGE,
                max_attack_range: CANNON_MAX_ATTACK_RANGE,
                rotation_angle: None,
            }
            .into(),
            Box::new(TargetProjectileAttack {
                damage: level.attack_damage,
                projectile_speed: CANNON_PROJECTILE_SPEED,
            }),
        );

        world
            .insert_one(
                id,
                BuildingFindTarget {
                    attack_air: false,
                    attack_ground: true,
                    rotation_angle: None,
                    min_attack_range: CANNON_MIN_ATTACK_RANGE,
                    max_attack_range: CANNON_MAX_ATTACK_RANGE,
                    min_housing_space: 0,
                },
            )
            .unwrap();
    }
}
