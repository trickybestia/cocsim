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
    buildings::utils::other_building::{
        default_attack_collider,
        default_pathfinding_collider,
    },
    consts::MAX_BUILDING_POS,
    game::features::{
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Team,
        },
        buildings::Building,
        health::Health,
        position::Position,
    },
};

struct WallLevel {
    pub health: f32,
}

const WALL_LEVELS_LEN: usize = 18;
const WALL_LEVEL_INDEX_MAX: usize = WALL_LEVELS_LEN - 1;
const WALL_LEVELS: [WallLevel; WALL_LEVELS_LEN] = [
    WallLevel { health: 300.0 },
    WallLevel { health: 500.0 },
    WallLevel { health: 700.0 },
    WallLevel { health: 900.0 },
    WallLevel { health: 1400.0 },
    WallLevel { health: 2000.0 },
    WallLevel { health: 2500.0 },
    WallLevel { health: 3000.0 },
    WallLevel { health: 3500.0 },
    WallLevel { health: 4000.0 },
    WallLevel { health: 5000.0 },
    WallLevel { health: 7000.0 },
    WallLevel { health: 9000.0 },
    WallLevel { health: 11000.0 },
    WallLevel { health: 12500.0 },
    WallLevel { health: 13500.0 },
    WallLevel { health: 14500.0 },
    WallLevel { health: 15500.0 },
];

const WALL: BuildingType = BuildingType {
    name: "Wall",
    size: Vector2::new(1, 1),
    levels: WALL_LEVELS.len(),
    options: &[],
};

inventory::submit! {WALL}

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct WallModel {
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<WALL_LEVEL_INDEX_MAX>,
}

impl BuildingModel for WallModel {
    fn r#type(&self) -> &'static BuildingType {
        &WALL
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn spawn(&self, world: &mut World) {
        world.spawn((
            Health(WALL_LEVELS[*self.level].health),
            Position(self.position().cast() + WALL.size.cast() / 2.0),
            Building {
                position: Vector2::new(*self.x, *self.y),
                size: WALL.size,
                affects_drop_zone: true,
                affects_percentage_destroyed: false,
            },
            default_pathfinding_collider(WALL.size),
            Team::Defense,
            AttackTarget {
                collider: default_attack_collider(WALL.size),
                flags: AttackTargetFlags::GROUND | AttackTargetFlags::WALL_BUILDING,
            },
        ));
    }
}
