use std::collections::HashSet;

use nalgebra::{
    DMatrix,
    Vector2,
};
use shipyard::{
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
    colliders::{
        Collider,
        ColliderEnum,
    },
    consts::*,
    game::{
        MapSize,
        features::position::Position,
    },
    utils::intersects,
};

#[derive(Component)]
#[track(All)]
pub struct PathfindingCollider(pub ColliderEnum);

#[derive(Unique)]
pub struct PathfindingCollisionGrid(pub DMatrix<EntityId>);

impl Default for PathfindingCollisionGrid {
    fn default() -> Self {
        unimplemented!()
    }
}

#[derive(Unique)]
pub struct NeedRedrawCollision(pub bool);

impl Default for NeedRedrawCollision {
    fn default() -> Self {
        unimplemented!()
    }
}

pub fn init_collision_grid(
    map_size: UniqueView<MapSize>,
    collision_grid: UniqueOrInitView<PathfindingCollisionGrid>,
    need_redraw_collision: UniqueOrInitView<NeedRedrawCollision>,
) {
    collision_grid
        .set(PathfindingCollisionGrid(DMatrix::from_element(
            map_size.total_size() as usize * COLLISION_TILES_PER_MAP_TILE,
            map_size.total_size() as usize * COLLISION_TILES_PER_MAP_TILE,
            EntityId::dead(),
        )))
        .unwrap();
    need_redraw_collision
        .set(NeedRedrawCollision(true))
        .unwrap();
}

pub fn update_collision(
    mut collision_grid: UniqueViewMut<PathfindingCollisionGrid>,
    mut need_redraw_collision: UniqueViewMut<NeedRedrawCollision>,
    v_position: View<Position>,
    v_collider: View<PathfindingCollider>,
) {
    if intersects(
        v_position.removed_or_deleted(),
        v_collider.removed_or_deleted(),
    ) {
        need_redraw_collision.0 = true;
    }

    let mut modified_ids = HashSet::new();

    for (id, _) in (&v_position, &v_collider).iter().with_id() {
        if v_position.is_inserted_or_modified(id) || v_collider.is_inserted_or_modified(id) {
            modified_ids.insert(id);
        }
    }

    if !modified_ids.is_empty() {
        for item in &mut collision_grid.0 {
            if *item != EntityId::dead() && modified_ids.contains(item) {
                *item = EntityId::dead();
            }
        }

        need_redraw_collision.0 = true;
    }

    for (id, (position, collider)) in (&v_position, &v_collider).iter().with_id() {
        if !(v_position.is_inserted_or_modified(id) || v_collider.is_inserted_or_modified(id)) {
            continue;
        }

        let collider = collider.0.translate(position.0);
        let bounding_box = collider.bounding_box();
        let start_x =
            (bounding_box.position.x * COLLISION_TILES_PER_MAP_TILE as f32).floor() as usize;
        let start_y =
            (bounding_box.position.y * COLLISION_TILES_PER_MAP_TILE as f32).floor() as usize;
        let end_x = ((bounding_box.position.x + bounding_box.size.x)
            * COLLISION_TILES_PER_MAP_TILE as f32)
            .ceil() as usize;
        let end_y = ((bounding_box.position.y + bounding_box.size.y)
            * COLLISION_TILES_PER_MAP_TILE as f32)
            .ceil() as usize;

        for x in start_x..=end_x {
            for y in start_y..=end_y {
                let occupy_tile = collider.contains(Vector2::new(
                    x as f32 / COLLISION_TILES_PER_MAP_TILE as f32,
                    y as f32 / COLLISION_TILES_PER_MAP_TILE as f32,
                ));

                if occupy_tile {
                    collision_grid.0[(x, y)] = id
                }
            }
        }
    }
}

pub fn cleanup_tracking(mut v_collider: ViewMut<PathfindingCollider>) {
    v_collider.clear_all_removed_and_deleted();
    v_collider.clear_all_inserted_and_modified();
}
