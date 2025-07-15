mod colliders;

pub use colliders::*;
use nalgebra::{
    DMatrix,
    Vector2,
};
use shipyard::{
    AddComponent,
    Component,
    EntityId,
    IntoIter,
    Unique,
    UniqueOrInitView,
    UniqueView,
    UniqueViewMut,
    View,
    ViewMut,
};

use crate::{
    consts::*,
    game::{
        MapSize,
        features::{
            buildings::Building,
            health::{
                DeathRequest,
                Health,
            },
        },
    },
};

#[derive(Component)]
pub struct ColliderComponent(pub ColliderEnum);

#[derive(Component)]
pub struct UpdateCollisionRequest;

#[derive(Unique)]
pub struct CollisionGrid(pub DMatrix<Option<EntityId>>);

impl Default for CollisionGrid {
    fn default() -> Self {
        unimplemented!()
    }
}

#[derive(Unique)]
pub struct NeedRedrawCollision(pub bool);

pub fn init_collision_grid(
    map_size: UniqueView<MapSize>,
    collision_grid: UniqueOrInitView<CollisionGrid>,
) {
    collision_grid
        .set(CollisionGrid(DMatrix::from_element(
            map_size.total_size() as usize * COLLISION_TILES_PER_MAP_TILE,
            map_size.total_size() as usize * COLLISION_TILES_PER_MAP_TILE,
            None,
        )))
        .unwrap();
}

pub fn request_update_collision_on_death_request(
    v_collider: View<ColliderComponent>,
    v_death_request: View<DeathRequest>,
    mut v_update_collision_request: ViewMut<UpdateCollisionRequest>,
) {
    for (id, _) in (&v_collider, &v_death_request).iter().with_id() {
        v_update_collision_request.add_component_unchecked(id, UpdateCollisionRequest);
    }
}

pub fn update_collision(
    mut collision_grid: UniqueViewMut<CollisionGrid>,
    mut need_redraw_collision: UniqueViewMut<NeedRedrawCollision>,
    v_building: View<Building>,
    v_collider: View<ColliderComponent>,
    v_health: View<Health>,
    mut v_update_collision_request: ViewMut<UpdateCollisionRequest>,
) {
    for (id, (building, collider, health, _)) in (
        &v_building,
        &v_collider,
        &v_health,
        &v_update_collision_request,
    )
        .iter()
        .with_id()
    {
        for rel_x in 0..(building.size.x * COLLISION_TILES_PER_MAP_TILE) {
            let abs_x = building.position.x * COLLISION_TILES_PER_MAP_TILE + rel_x;

            for rel_y in 0..building.size.y {
                let abs_y = building.position.y * COLLISION_TILES_PER_MAP_TILE + rel_y;

                let occupy_tile = health.0 > 0.0
                    && collider.0.contains(Vector2::new(
                        abs_x as f32 / COLLISION_TILES_PER_MAP_TILE as f32,
                        abs_y as f32 / COLLISION_TILES_PER_MAP_TILE as f32,
                    ));

                collision_grid.0[(abs_x, abs_y)] = if occupy_tile { Some(id) } else { None }
            }
        }

        need_redraw_collision.0 = true;
    }

    v_update_collision_request.clear();
}
