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
    UsizeWithMax,
    buildings::utils::active_building::create_active_building,
    consts::MAX_BUILDING_POS,
    game::features::actions::{
        BuildingFindTarget,
        TargetProjectileAttack,
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
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<ARCHER_TOWER_LEVEL_INDEX_MAX>,
}

impl BuildingModel for ArcherTowerModel {
    fn r#type(&self) -> &'static BuildingType {
        &ARCHER_TOWER
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn create_building(&self, world: &mut World) {
        let level = &ARCHER_TOWER_LEVELS[*self.level];

        create_active_building(
            world,
            level.health,
            Vector2::new(*self.x, *self.y),
            ARCHER_TOWER.size,
            ARCHER_TOWER_MIN_ATTACK_RANGE,
            ARCHER_TOWER_MAX_ATTACK_RANGE,
            ARCHER_TOWER_ATTACK_COOLDOWN,
            BuildingFindTarget {
                attack_air: true,
                attack_ground: true,
                rotation_angle: None,
            }
            .into(),
            TargetProjectileAttack {
                damage: level.attack_damage,
                projectile_speed: ARCHER_TOWER_PROJECTILE_SPEED,
            }
            .into(),
        );
    }
}
