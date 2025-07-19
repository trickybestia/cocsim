mod colliders;

pub use colliders::*;
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
    ViewMut,
    track::InsertionAndModification,
};

use crate::{
    consts::*,
    game::MapSize,
};

pub struct ColliderComponent(pub ColliderEnum);

impl Component for ColliderComponent {
    type Tracking = InsertionAndModification;
}

#[derive(Unique)]
pub struct CollisionGrid(pub DMatrix<EntityId>);

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
            EntityId::dead(),
        )))
        .unwrap();
}

pub fn update_collision(
    mut collision_grid: UniqueViewMut<CollisionGrid>,
    mut need_redraw_collision: UniqueViewMut<NeedRedrawCollision>,
    v_collider: ViewMut<ColliderComponent>,
) {
    let modified_ids = v_collider
        .modified()
        .iter()
        .with_id()
        .map(|(id, _)| id)
        .collect::<Vec<_>>();

    if !modified_ids.is_empty() {
        for item in &mut collision_grid.0 {
            if modified_ids.contains(item) {
                *item = EntityId::dead();
            }
        }
    }

    for (id, collider) in v_collider.inserted_or_modified().iter().with_id() {
        let bounding_box = collider.0.bounding_box();
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

                let occupy_tile = collider.0.contains(Vector2::new(
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

pub fn cleanup_tracking(v_collider: ViewMut<ColliderComponent>) {
    v_collider.clear_all_inserted_and_modified();
}
