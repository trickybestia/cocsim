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
    buildings::utils::active_building::create_active_building,
    consts::MAX_BUILDING_POS,
    game::features::{
        actions::{
            Delayed,
            SplashDamage,
            SplashProjectileAttack,
        },
        attack::BuildingRetargetCondition,
        targeting::building::BuildingFindTarget,
        to_be_deleted::OnDelete,
    },
};

struct BombTowerLevel {
    pub health: f32,
    pub attack_damage: f32,
    pub death_damage: f32,
}

const BOMB_TOWER_LEVELS_LEN: usize = 12;
const BOMB_TOWER_LEVEL_INDEX_MAX: usize = BOMB_TOWER_LEVELS_LEN - 1;
const BOMB_TOWER_LEVELS: [BombTowerLevel; BOMB_TOWER_LEVELS_LEN] = [
    BombTowerLevel {
        health: 650.0,
        attack_damage: 26.4,
        death_damage: 150.0,
    },
    BombTowerLevel {
        health: 700.0,
        attack_damage: 30.8,
        death_damage: 180.0,
    },
    BombTowerLevel {
        health: 750.0,
        attack_damage: 35.2,
        death_damage: 220.0,
    },
    BombTowerLevel {
        health: 850.0,
        attack_damage: 44.0,
        death_damage: 260.0,
    },
    BombTowerLevel {
        health: 1050.0,
        attack_damage: 52.8,
        death_damage: 300.0,
    },
    BombTowerLevel {
        health: 1300.0,
        attack_damage: 61.6,
        death_damage: 350.0,
    },
    BombTowerLevel {
        health: 1600.0,
        attack_damage: 70.4,
        death_damage: 400.0,
    },
    BombTowerLevel {
        health: 1900.0,
        attack_damage: 79.2,
        death_damage: 450.0,
    },
    BombTowerLevel {
        health: 2300.0,
        attack_damage: 92.4,
        death_damage: 500.0,
    },
    BombTowerLevel {
        health: 2500.0,
        attack_damage: 103.4,
        death_damage: 550.0,
    },
    BombTowerLevel {
        health: 2700.0,
        attack_damage: 114.4,
        death_damage: 600.0,
    },
    BombTowerLevel {
        health: 2900.0,
        attack_damage: 125.4,
        death_damage: 650.0,
    },
];

const BOMB_TOWER: BuildingType = BuildingType {
    name: "BombTower",
    size: Vector2::new(3, 3),
    levels: BOMB_TOWER_LEVELS.len(),
    options: &[],
};

inventory::submit! {BOMB_TOWER}

const BOMB_TOWER_MIN_ATTACK_RANGE: f32 = 0.0;
const BOMB_TOWER_MAX_ATTACK_RANGE: f32 = 6.0;
const BOMB_TOWER_ATTACK_COOLDOWN: f32 = 1.1;
const BOMB_TOWER_PROJECTILE_SPEED: f32 = 8.0;
const BOMB_TOWER_SPLASH_ATTACK_RADIUS: f32 = 1.5;
const BOMB_TOWER_DEATH_DAMAGE_ATTACK_RADIUS: f32 = 2.75;
const BOMB_TOWER_DEATH_DAMAGE_DELAY: f32 = 1.0;

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct BombTowerModel {
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<BOMB_TOWER_LEVEL_INDEX_MAX>,
}

impl BuildingModel for BombTowerModel {
    fn r#type(&self) -> &'static BuildingType {
        &BOMB_TOWER
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn create_building(&self, world: &mut World) {
        let level = &BOMB_TOWER_LEVELS[*self.level];

        let id = create_active_building(
            world,
            level.health,
            Vector2::new(*self.x, *self.y),
            BOMB_TOWER.size,
            BOMB_TOWER_ATTACK_COOLDOWN,
            BuildingRetargetCondition {
                min_attack_range: BOMB_TOWER_MIN_ATTACK_RANGE,
                max_attack_range: BOMB_TOWER_MAX_ATTACK_RANGE,
                rotation_angle: None,
            }
            .into(),
            SplashProjectileAttack {
                damage: level.attack_damage,
                damage_radius: BOMB_TOWER_SPLASH_ATTACK_RADIUS,
                damage_air: false,
                damage_ground: true,
                projectile_speed: BOMB_TOWER_PROJECTILE_SPEED,
            }
            .into(),
        );

        world
            .insert(
                id,
                (
                    BuildingFindTarget {
                        attack_air: false,
                        attack_ground: true,
                        rotation_angle: None,
                        min_attack_range: BOMB_TOWER_MIN_ATTACK_RANGE,
                        max_attack_range: BOMB_TOWER_MAX_ATTACK_RANGE,
                    },
                    OnDelete(
                        Delayed {
                            time: BOMB_TOWER_DEATH_DAMAGE_DELAY,
                            action: Box::new(
                                SplashDamage {
                                    damage_ground: true,
                                    damage_air: false,
                                    damage: level.death_damage,
                                    radius: BOMB_TOWER_DEATH_DAMAGE_ATTACK_RADIUS,
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                ),
            )
            .unwrap();
    }
}
