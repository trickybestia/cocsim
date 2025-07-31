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
    consts::*,
    game::{
        MapSize,
        features::buildings::Building,
    },
    utils::intersects,
};

#[derive(Component)]
#[track(All)]
pub struct PathfindingCollider {
    /// position relative to top-left corner of building
    pub position: Vector2<usize>,
    pub size: Vector2<usize>,
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
    v_building: View<Building>,
    v_collider: View<PathfindingCollider>,
) {
    if intersects(
        v_building.removed_or_deleted(),
        v_collider.removed_or_deleted(),
    ) {
        need_redraw_collision.0 = true;
    }

    let mut modified_ids = HashSet::new();

    for (id, _) in (&v_building, &v_collider).iter().with_id() {
        if v_building.is_inserted_or_modified(id) || v_collider.is_inserted_or_modified(id) {
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

    for (id, (building, collider)) in (&v_building, &v_collider).iter().with_id() {
        if !(v_building.is_inserted_or_modified(id) || v_collider.is_inserted_or_modified(id)) {
            continue;
        }

        let start = building.position * COLLISION_TILES_PER_MAP_TILE + collider.position;
        let end_exclusive = start + collider.size;

        for x in start.x..end_exclusive.x {
            for y in start.y..end_exclusive.y {
                collision_grid.0[(x, y)] = id
            }
        }
    }
}

pub fn cleanup_tracking(mut v_collider: ViewMut<PathfindingCollider>) {
    v_collider.clear_all_removed_and_deleted();
    v_collider.clear_all_inserted_and_modified();
}
