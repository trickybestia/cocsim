use hecs::{
    Entity,
    PreparedQuery,
    With,
    World,
};
use nalgebra::{
    DMatrix,
    Vector2,
};

use crate::{
    Game,
    consts::*,
    game::{
        MapSize,
        features::{
            buildings::Building,
            to_be_despawned::ToBeDespawned,
        },
    },
    utils::AnyMapExt,
};

pub struct PathfindingCollider {
    /// position relative to top-left corner of building
    pub position: Vector2<usize>,
    pub size: Vector2<usize>,
}

pub struct PathfindingCollisionGrid(pub DMatrix<Entity>);

impl PathfindingCollisionGrid {
    pub fn new(map_size: &MapSize, world: &World) -> Self {
        let mut result = DMatrix::from_element(
            map_size.total_size() as usize * COLLISION_TILES_PER_MAP_TILE,
            map_size.total_size() as usize * COLLISION_TILES_PER_MAP_TILE,
            Entity::DANGLING,
        );

        for (id, (building, collider)) in world.query::<(&Building, &PathfindingCollider)>().iter()
        {
            let start = building.position * COLLISION_TILES_PER_MAP_TILE + collider.position;
            let end_exclusive = start + collider.size;

            for x in start.x..end_exclusive.x {
                for y in start.y..end_exclusive.y {
                    result[(x, y)] = id;
                }
            }
        }

        Self(result)
    }
}

pub fn check_need_redraw_collision(game: &mut Game) {
    if game
        .cache
        .get_mut_or_default::<PreparedQuery<With<(&Building, &PathfindingCollider), &ToBeDespawned>>>(
        )
        .query_mut(&mut game.world)
        .into_iter()
        .count()
        != 0
    {
        game.need_redraw_collision = true;
    }
}
