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
    track::InsertionAndModification,
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
};

pub struct PathfindingCollider(pub ColliderEnum);

impl Component for PathfindingCollider {
    type Tracking = InsertionAndModification;
}

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
    let mut modified_ids = Vec::new();

    for (id, _) in (&v_position, &v_collider).iter().with_id() {
        if v_position.is_inserted_or_modified(id) || v_collider.is_inserted_or_modified(id) {
            modified_ids.push(id);
        }
    }

    if !modified_ids.is_empty() {
        for item in &mut collision_grid.0 {
            if modified_ids.contains(item) {
                *item = EntityId::dead();
            }
        }
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

        for rel_x in
            0..((bounding_box.size.x * COLLISION_TILES_PER_MAP_TILE as f32).ceil() as usize)
        {
            let abs_x = start_x + rel_x;

            for rel_y in
                0..(bounding_box.size.y * COLLISION_TILES_PER_MAP_TILE as f32).ceil() as usize
            {
                let abs_y = start_y + rel_y;

                let occupy_tile = collider.contains(Vector2::new(
                    abs_x as f32 / COLLISION_TILES_PER_MAP_TILE as f32,
                    abs_y as f32 / COLLISION_TILES_PER_MAP_TILE as f32,
                ));

                if occupy_tile {
                    collision_grid.0[(abs_x, abs_y)] = id
                }
            }
        }

        need_redraw_collision.0 = true;
    }
}

pub fn cleanup_tracking(v_collider: ViewMut<PathfindingCollider>) {
    v_collider.clear_all_inserted_and_modified();
}
