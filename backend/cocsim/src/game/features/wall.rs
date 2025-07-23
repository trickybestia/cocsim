use std::collections::HashSet;

use nalgebra::Vector2;
use shipyard::{
    Component,
    Get,
    IntoIter,
    UniqueView,
    View,
    ViewMut,
};

use crate::{
    colliders::{
        ListCollider,
        RectCollider,
    },
    game::features::{
        buildings::{
            Building,
            BuildingsGrid,
        },
        collision::PathfindingCollider,
        health::ToBeDeleted,
        map::MapSize,
    },
};

#[derive(Component)]
#[track(Insertion)]
pub struct Wall {
    pub center_collider: RectCollider,
    pub up_connection_collider: RectCollider,
    pub down_connection_collider: RectCollider,
    pub left_connection_collider: RectCollider,
    pub right_connection_collider: RectCollider,
}

const WALL_NEIGHBORS_OFFSETS: [Vector2<i32>; 4] = [
    Vector2::new(0, 1),
    Vector2::new(1, 0),
    Vector2::new(0, -1),
    Vector2::new(-1, 0),
];

pub fn update_walls(
    map_size: UniqueView<MapSize>,
    buildings_grid: UniqueView<BuildingsGrid>,
    v_building: View<Building>,
    mut v_collider: ViewMut<PathfindingCollider>,
    v_wall: View<Wall>,
    v_to_be_deleted: View<ToBeDeleted>,
) {
    let mut positions_to_update = HashSet::new();

    for (id, (building, _)) in (&v_building, &v_wall).iter().with_id() {
        if v_wall.is_inserted(id) || v_to_be_deleted.contains(id) {
            positions_to_update.insert((building.position.x as i32, building.position.y as i32));

            for offset in WALL_NEIGHBORS_OFFSETS {
                let neighbor_position = building.position.cast() + offset;

                if map_size.is_inside_map(neighbor_position) {
                    positions_to_update.insert((neighbor_position.x, neighbor_position.y));
                }
            }
        }
    }

    for position in positions_to_update.into_iter() {
        let wall_id = buildings_grid.0[(position.0 as usize, position.1 as usize)];
        let wall = v_wall.get(wall_id);

        if let Ok(wall) = wall {
            let mut colliders = vec![wall.center_collider.clone().into()];

            {
                let up_tile_position = Vector2::new(position.0, position.1 - 1);

                if map_size.is_inside_map(up_tile_position) {
                    let neighbor_id = buildings_grid.0
                        [(up_tile_position.x as usize, up_tile_position.y as usize)];

                    if v_wall.contains(neighbor_id) && !v_to_be_deleted.contains(neighbor_id) {
                        colliders.push(wall.up_connection_collider.clone().into());
                    }
                }
            }
            {
                let down_tile_position = Vector2::new(position.0, position.1 + 1);

                if map_size.is_inside_map(down_tile_position) {
                    let neighbor_id = buildings_grid.0
                        [(down_tile_position.x as usize, down_tile_position.y as usize)];

                    if v_wall.contains(neighbor_id) && !v_to_be_deleted.contains(neighbor_id) {
                        colliders.push(wall.down_connection_collider.clone().into());
                    }
                }
            }
            {
                let left_tile_position = Vector2::new(position.0 - 1, position.1);

                if map_size.is_inside_map(left_tile_position) {
                    let neighbor_id = buildings_grid.0
                        [(left_tile_position.x as usize, left_tile_position.y as usize)];

                    if v_wall.contains(neighbor_id) && !v_to_be_deleted.contains(neighbor_id) {
                        colliders.push(wall.left_connection_collider.clone().into());
                    }
                }
            }
            {
                let right_tile_position = Vector2::new(position.0 + 1, position.1);

                if map_size.is_inside_map(right_tile_position) {
                    let neighbor_id = buildings_grid.0[(
                        right_tile_position.x as usize,
                        right_tile_position.y as usize,
                    )];

                    if v_wall.contains(neighbor_id) && !v_to_be_deleted.contains(neighbor_id) {
                        colliders.push(wall.right_connection_collider.clone().into());
                    }
                }
            }

            v_collider[wall_id].0 = ListCollider::new(colliders).into();
        }
    }
}

pub fn cleanup_tracking(v_wall: ViewMut<Wall>) {
    v_wall.clear_all_inserted();
}
