use anyhow::{
    Context,
    Result,
};
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
    buildings::utils::passive_building::default_attack_collider,
    colliders::{
        Collider,
        RectCollider,
    },
    game::features::{
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Team,
        },
        buildings::Building,
        collision::PathfindingCollider,
        health::Health,
        wall::Wall,
    },
};

struct WallLevel {
    pub health: f32,
}

const WALL_LEVELS: &[WallLevel] = &[
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

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub struct WallModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for WallModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        let collider = default_attack_collider(WALL.size);

        world.add_entity((
            Health(
                WALL_LEVELS
                    .get(self.level)
                    .context("Level out of range")?
                    .health,
            ),
            Building {
                position: Vector2::new(self.x, self.y),
                size: WALL.size,
            },
            PathfindingCollider(collider.clone()),
            Team::Defense,
            AttackTarget {
                collider,
                flags: AttackTargetFlags::GROUND | AttackTargetFlags::BUILDING,
            },
            Wall {
                center_collider: RectCollider::new_from_center(
                    Vector2::zeros(),
                    Vector2::from_element(0.65),
                ),
                up_connection_collider: RectCollider::new(
                    Vector2::new(0.175, 0.0),
                    Vector2::new(0.65, 0.5),
                )
                .translate(Vector2::from_element(-0.5)),
                down_connection_collider: RectCollider::new(
                    Vector2::new(0.175, 0.5),
                    Vector2::new(0.65, 0.5),
                )
                .translate(Vector2::from_element(-0.5)),
                left_connection_collider: RectCollider::new(
                    Vector2::new(0.0, 0.175),
                    Vector2::new(0.5, 0.65),
                )
                .translate(Vector2::from_element(-0.5)),
                right_connection_collider: RectCollider::new(
                    Vector2::new(0.5, 0.175),
                    Vector2::new(0.5, 0.65),
                )
                .translate(Vector2::from_element(-0.5)),
            },
        ));

        Ok(())
    }
}
