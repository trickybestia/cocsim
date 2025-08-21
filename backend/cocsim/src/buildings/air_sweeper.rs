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
        actions::AirSweeperAttack,
        attack::{
            BuildingRetargetCondition,
            targeting::building::BuildingFindTarget,
        },
    },
};

struct AirSweeperLevel {
    pub health: f32,
    pub push_strength: f32,
}

const AIR_SWEEPER_LEVELS_LEN: usize = 7;
const AIR_SWEEPER_LEVEL_INDEX_MAX: usize = AIR_SWEEPER_LEVELS_LEN - 1;
const AIR_SWEEPER_LEVELS: [AirSweeperLevel; AIR_SWEEPER_LEVELS_LEN] = [
    AirSweeperLevel {
        health: 750.0,
        push_strength: 1.6,
    },
    AirSweeperLevel {
        health: 800.0,
        push_strength: 2.0,
    },
    AirSweeperLevel {
        health: 850.0,
        push_strength: 2.4,
    },
    AirSweeperLevel {
        health: 900.0,
        push_strength: 2.8,
    },
    AirSweeperLevel {
        health: 950.0,
        push_strength: 3.2,
    },
    AirSweeperLevel {
        health: 1000.0,
        push_strength: 3.6,
    },
    AirSweeperLevel {
        health: 1050.0,
        push_strength: 4.0,
    },
];

const AIR_SWEEPER: BuildingType = BuildingType {
    name: "AirSweeper",
    size: Vector2::new(2, 2),
    levels: AIR_SWEEPER_LEVELS.len(),
    options: &[BuildingOption {
        name: "rotation",
        values: &[
            "Right",
            "RightUp",
            "Up",
            "LeftUp",
            "Left",
            "LeftDown",
            "Down",
            "RightDown",
        ],
    }],
    affects_drop_zone: true,
};

inventory::submit! {AIR_SWEEPER}

const AIR_SWEEPER_MIN_ATTACK_RANGE: f32 = 1.0;
const AIR_SWEEPER_MAX_ATTACK_RANGE: f32 = 15.0;
const AIR_SWEEPER_ATTACK_COOLDOWN: f32 = 5.0;
const AIR_SWEEPER_PROJECTILE_SPEED: f32 = 6.0;
/// https://clashofclans.fandom.com/wiki/Air_Sweeper#Trivia
const AIR_SWEEPER_PROJECTILE_ANGLE: f32 = 60.0;
/// https://clashofclans.fandom.com/wiki/Air_Sweeper#Trivia
const AIR_SWEEPER_PROJECTILE_MAX_ARC_LENGTH: f32 = 5.0;

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub enum AirSweeperRotation {
    Right,
    RightUp,
    Up,
    LeftUp,
    Left,
    LeftDown,
    Down,
    RightDown,
}

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct AirSweeperModel {
    pub level: UsizeWithMax<AIR_SWEEPER_LEVEL_INDEX_MAX>,
    pub rotation: AirSweeperRotation,
}

impl BuildingModel for AirSweeperModel {
    fn r#type(&self) -> &'static BuildingType {
        &AIR_SWEEPER
    }

    fn spawn(&self, world: &mut World, position: Vector2<usize>) {
        let level = &AIR_SWEEPER_LEVELS[*self.level];
        let rotation = match self.rotation {
            AirSweeperRotation::Right => 0.0,
            AirSweeperRotation::RightUp => 315.0,
            AirSweeperRotation::Up => 270.0,
            AirSweeperRotation::LeftUp => 225.0,
            AirSweeperRotation::Left => 180.0,
            AirSweeperRotation::LeftDown => 135.0,
            AirSweeperRotation::Down => 90.0,
            AirSweeperRotation::RightDown => 45.0,
        };
        let rotation_angle = Some((rotation - 60.0, 120.0));

        let id = spawn_defensive_building(
            world,
            level.health,
            position,
            AIR_SWEEPER.size,
            AIR_SWEEPER_ATTACK_COOLDOWN,
            BuildingRetargetCondition {
                min_attack_range: AIR_SWEEPER_MIN_ATTACK_RANGE,
                max_attack_range: AIR_SWEEPER_MAX_ATTACK_RANGE,
                rotation_angle,
            }
            .into(),
            Box::new(AirSweeperAttack {
                push_strength: level.push_strength,
                projectile_speed: AIR_SWEEPER_PROJECTILE_SPEED,
                start_radius: AIR_SWEEPER_MIN_ATTACK_RANGE,
                max_radius: AIR_SWEEPER_MAX_ATTACK_RANGE,
                angle: AIR_SWEEPER_PROJECTILE_ANGLE,
                max_arc_length: AIR_SWEEPER_PROJECTILE_MAX_ARC_LENGTH,
            }),
        );

        world
            .insert_one(
                id,
                BuildingFindTarget {
                    attack_air: true,
                    attack_ground: false,
                    rotation_angle,
                    min_attack_range: AIR_SWEEPER_MIN_ATTACK_RANGE,
                    max_attack_range: AIR_SWEEPER_MAX_ATTACK_RANGE,
                    min_housing_space: 0,
                },
            )
            .unwrap();
    }
}
