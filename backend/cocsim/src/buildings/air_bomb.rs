use std::sync::Arc;

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
    buildings::utils::trap::spawn_trap,
    consts::MAX_BUILDING_POS,
    game::features::{
        actions::SplashProjectileAttack,
        attack::targeting::building::BuildingFindTarget,
    },
};

struct AirBombLevel {
    pub damage: f32,
}

const AIR_BOMB_LEVELS_LEN: usize = 12;
const AIR_BOMB_LEVEL_INDEX_MAX: usize = AIR_BOMB_LEVELS_LEN - 1;
const AIR_BOMB_LEVELS: [AirBombLevel; AIR_BOMB_LEVELS_LEN] = [
    AirBombLevel { damage: 100.0 },
    AirBombLevel { damage: 120.0 },
    AirBombLevel { damage: 144.0 },
    AirBombLevel { damage: 173.0 },
    AirBombLevel { damage: 208.0 },
    AirBombLevel { damage: 232.0 },
    AirBombLevel { damage: 252.0 },
    AirBombLevel { damage: 280.0 },
    AirBombLevel { damage: 325.0 },
    AirBombLevel { damage: 350.0 },
    AirBombLevel { damage: 375.0 },
    AirBombLevel { damage: 400.0 },
];

const AIR_BOMB: BuildingType = BuildingType {
    name: "AirBomb",
    size: Vector2::new(1, 1),
    levels: AIR_BOMB_LEVELS.len(),
    options: &[],
};

inventory::submit! {AIR_BOMB}

const AIR_BOMB_TRIGGER_RADIUS: f32 = 4.0;
const AIR_BOMB_DAMAGE_RADIUS: f32 = 3.0;
const AIR_BOMB_SPEED: f32 = 2.5;

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct AirBombModel {
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<AIR_BOMB_LEVEL_INDEX_MAX>,
}

impl BuildingModel for AirBombModel {
    fn r#type(&self) -> &'static BuildingType {
        &AIR_BOMB
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn spawn(&self, world: &mut World) {
        let id = spawn_trap(
            world,
            self.position(),
            AIR_BOMB.size,
            Arc::new(SplashProjectileAttack {
                damage: AIR_BOMB_LEVELS[*self.level].damage,
                damage_radius: AIR_BOMB_DAMAGE_RADIUS,
                damage_air: true,
                damage_ground: false,
                projectile_speed: AIR_BOMB_SPEED,
            }),
        );

        world
            .insert_one(
                id,
                BuildingFindTarget {
                    attack_air: true,
                    attack_ground: false,
                    rotation_angle: None,
                    min_attack_range: 0.0,
                    max_attack_range: AIR_BOMB_TRIGGER_RADIUS,
                    min_housing_space: 0,
                },
            )
            .unwrap();
    }
}
