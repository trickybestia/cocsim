use arbitrary::Arbitrary;
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
    UsizeWithMax,
    buildings::utils::active_building::create_active_building,
    consts::MAX_BUILDING_POS,
    game::features::{
        actions::AirSweeperAttack,
        attack::BuildingRetargetCondition,
        targeting::building::BuildingFindTarget,
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
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<AIR_SWEEPER_LEVEL_INDEX_MAX>,
    pub rotation: AirSweeperRotation,
}

impl BuildingModel for AirSweeperModel {
    fn r#type(&self) -> &'static BuildingType {
        &AIR_SWEEPER
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn create_building(&self, world: &mut World) {
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

        let id = create_active_building(
            world,
            level.health,
            self.position(),
            AIR_SWEEPER.size,
            AIR_SWEEPER_ATTACK_COOLDOWN,
            BuildingRetargetCondition {
                min_attack_range: AIR_SWEEPER_MIN_ATTACK_RANGE,
                max_attack_range: AIR_SWEEPER_MAX_ATTACK_RANGE,
                rotation_angle,
            }
            .into(),
            AirSweeperAttack {
                push_strength: level.push_strength,
                projectile_speed: AIR_SWEEPER_PROJECTILE_SPEED,
                start_radius: AIR_SWEEPER_MIN_ATTACK_RANGE,
                max_radius: AIR_SWEEPER_MAX_ATTACK_RANGE,
                angle: AIR_SWEEPER_PROJECTILE_ANGLE,
                max_arc_length: AIR_SWEEPER_PROJECTILE_MAX_ARC_LENGTH,
            }
            .into(),
        );

        world.add_component(
            id,
            BuildingFindTarget {
                attack_air: true,
                attack_ground: false,
                rotation_angle,
                min_attack_range: AIR_SWEEPER_MIN_ATTACK_RANGE,
                max_attack_range: AIR_SWEEPER_MAX_ATTACK_RANGE,
            },
        );
    }
}
