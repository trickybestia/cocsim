use arbitrary::Arbitrary;
use hecs::World;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    BuildingModel,
    BuildingOption,
    BuildingType,
    UsizeWithMax,
    buildings::utils::defensive_building::spawn_defensive_building,
    game::features::{
        actions::TargetProjectileAttack,
        attack::{
            BuildingRetargetCondition,
            targeting::building::BuildingFindTarget,
        },
    },
};

struct XBowLevel {
    pub health: f32,
    pub attack_damage: f32,
}

const X_BOW_LEVELS_LEN: usize = 12;
const X_BOW_LEVEL_INDEX_MAX: usize = X_BOW_LEVELS_LEN - 1;
const X_BOW_LEVELS: [XBowLevel; X_BOW_LEVELS_LEN] = [
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

const X_BOW_MIN_ATTACK_RANGE: f32 = 0.0;
const X_BOW_ATTACK_COOLDOWN: f32 = 0.128;

#[derive(Serialize, Deserialize, Debug, Arbitrary, PartialEq, Eq, Clone, Copy)]
pub enum XBowTargetType {
    Ground,
    AirAndGround,
}

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct XBowModel {
    pub level: UsizeWithMax<X_BOW_LEVEL_INDEX_MAX>,
    pub target: XBowTargetType,
}

impl BuildingModel for XBowModel {
    fn r#type(&self) -> &'static BuildingType {
        &X_BOW
    }

    fn spawn(&self, world: &mut World, position: Vector2<usize>) {
        let max_attack_range = match self.target {
            XBowTargetType::Ground => 14.0,
            XBowTargetType::AirAndGround => 11.5,
        };
        let projectile_speed = match *self.level {
            0 => 23.0,
            1 => 24.0,
            _ => 25.0,
        };
        let level = &X_BOW_LEVELS[*self.level];

        let id = spawn_defensive_building(
            world,
            level.health,
            position,
            X_BOW.size,
            X_BOW_ATTACK_COOLDOWN,
            BuildingRetargetCondition {
                min_attack_range: X_BOW_MIN_ATTACK_RANGE,
                max_attack_range,
                rotation_angle: None,
            }
            .into(),
            Box::new(TargetProjectileAttack {
                damage: level.attack_damage,
                projectile_speed,
            }),
        );

        world
            .insert_one(
                id,
                BuildingFindTarget {
                    attack_air: self.target == XBowTargetType::AirAndGround,
                    attack_ground: true,
                    rotation_angle: None,
                    min_attack_range: X_BOW_MIN_ATTACK_RANGE,
                    max_attack_range,
                    min_housing_space: 0,
                },
            )
            .unwrap();
    }
}
