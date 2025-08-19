use arbitrary::Arbitrary;
use hecs::World;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    BuildingModel,
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

struct ArcherTowerLevel {
    pub health: f32,
    pub attack_damage: f32,
}

const ARCHER_TOWER_LEVELS_LEN: usize = 21;
const ARCHER_TOWER_LEVEL_INDEX_MAX: usize = ARCHER_TOWER_LEVELS_LEN - 1;
const ARCHER_TOWER_LEVELS: [ArcherTowerLevel; ARCHER_TOWER_LEVELS_LEN] = [
    ArcherTowerLevel {
        health: 380.0,
        attack_damage: 5.5,
    },
    ArcherTowerLevel {
        health: 420.0,
        attack_damage: 7.5,
    },
    ArcherTowerLevel {
        health: 460.0,
        attack_damage: 9.5,
    },
    ArcherTowerLevel {
        health: 500.0,
        attack_damage: 12.5,
    },
    ArcherTowerLevel {
        health: 540.0,
        attack_damage: 15.0,
    },
    ArcherTowerLevel {
        health: 580.0,
        attack_damage: 17.5,
    },
    ArcherTowerLevel {
        health: 630.0,
        attack_damage: 21.0,
    },
    ArcherTowerLevel {
        health: 690.0,
        attack_damage: 24.0,
    },
    ArcherTowerLevel {
        health: 750.0,
        attack_damage: 28.0,
    },
    ArcherTowerLevel {
        health: 810.0,
        attack_damage: 31.5,
    },
    ArcherTowerLevel {
        health: 890.0,
        attack_damage: 35.0,
    },
    ArcherTowerLevel {
        health: 970.0,
        attack_damage: 37.0,
    },
    ArcherTowerLevel {
        health: 1050.0,
        attack_damage: 39.0,
    },
    ArcherTowerLevel {
        health: 1130.0,
        attack_damage: 41.0,
    },
    ArcherTowerLevel {
        health: 1230.0,
        attack_damage: 42.5,
    },
    ArcherTowerLevel {
        health: 1310.0,
        attack_damage: 45.0,
    },
    ArcherTowerLevel {
        health: 1390.0,
        attack_damage: 50.0,
    },
    ArcherTowerLevel {
        health: 1510.0,
        attack_damage: 55.0,
    },
    ArcherTowerLevel {
        health: 1600.0,
        attack_damage: 60.0,
    },
    ArcherTowerLevel {
        health: 1700.0,
        attack_damage: 67.5,
    },
    ArcherTowerLevel {
        health: 1800.0,
        attack_damage: 72.5,
    },
];

const ARCHER_TOWER: BuildingType = BuildingType {
    name: "ArcherTower",
    size: Vector2::new(3, 3),
    levels: ARCHER_TOWER_LEVELS.len(),
    options: &[],
};

inventory::submit! {ARCHER_TOWER}

const ARCHER_TOWER_MIN_ATTACK_RANGE: f32 = 0.0;
const ARCHER_TOWER_MAX_ATTACK_RANGE: f32 = 10.0;
const ARCHER_TOWER_ATTACK_COOLDOWN: f32 = 0.5;
const ARCHER_TOWER_PROJECTILE_SPEED: f32 = 18.0;

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct ArcherTowerModel {
    pub level: UsizeWithMax<ARCHER_TOWER_LEVEL_INDEX_MAX>,
}

impl BuildingModel for ArcherTowerModel {
    fn r#type(&self) -> &'static BuildingType {
        &ARCHER_TOWER
    }

    fn spawn(&self, world: &mut World, position: Vector2<usize>) {
        let level = &ARCHER_TOWER_LEVELS[*self.level];

        let id = spawn_defensive_building(
            world,
            level.health,
            position,
            ARCHER_TOWER.size,
            ARCHER_TOWER_ATTACK_COOLDOWN,
            BuildingRetargetCondition {
                min_attack_range: ARCHER_TOWER_MIN_ATTACK_RANGE,
                max_attack_range: ARCHER_TOWER_MAX_ATTACK_RANGE,
                rotation_angle: None,
            }
            .into(),
            Box::new(TargetProjectileAttack {
                damage: level.attack_damage,
                projectile_speed: ARCHER_TOWER_PROJECTILE_SPEED,
            }),
        );

        world
            .insert_one(
                id,
                BuildingFindTarget {
                    attack_air: true,
                    attack_ground: true,
                    rotation_angle: None,
                    min_attack_range: ARCHER_TOWER_MIN_ATTACK_RANGE,
                    max_attack_range: ARCHER_TOWER_MAX_ATTACK_RANGE,
                    min_housing_space: 0,
                },
            )
            .unwrap();
    }
}
