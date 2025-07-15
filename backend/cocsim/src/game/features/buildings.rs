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
    View,
};

use crate::game::features::map::MapSize;

#[derive(Component)]
pub struct Building {
    pub position: Vector2<usize>,
    pub size: Vector2<usize>,
}

/// "Counted" means that this building impacts destroyed buildings percentage.
#[derive(Component)]
pub struct CountedBuilding;

#[derive(Component)]
pub struct TownHall;

#[derive(Unique)]
pub struct BuildingsGrid(pub DMatrix<Option<EntityId>>);

impl Default for BuildingsGrid {
    fn default() -> Self {
        unimplemented!()
    }
}

#[derive(Unique)]
pub struct DropZone(pub DMatrix<bool>);

impl Default for DropZone {
    fn default() -> Self {
        unimplemented!()
    }
}

pub fn init_buildings_grid(
    map_size: UniqueView<MapSize>,
    buildings_grid: UniqueOrInitView<BuildingsGrid>,
    v_building: View<Building>,
) {
    let mut result = DMatrix::from_element(
        map_size.total_size() as usize,
        map_size.total_size() as usize,
        None,
    );

    for (id, building) in v_building.iter().with_id() {
        for rel_x in 0..building.size.x {
            let abs_x = building.position.x + rel_x;

            for rel_y in 0..building.size.y {
                let abs_y = building.position.y + rel_y;

                result[(abs_x, abs_y)] = Some(id)
            }
        }
    }

    buildings_grid.set(BuildingsGrid(result)).unwrap();
}

pub fn init_drop_zone(
    map_size: UniqueView<MapSize>,
    buildings_grid: UniqueView<BuildingsGrid>,
    drop_zone: UniqueOrInitView<DropZone>,
) {
    fn get_neighbors(map_size: &MapSize, x: i32, y: i32) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        for neighbor_x in (x - 1)..(x + 2) {
            for neighbor_y in (y - 1)..(y + 2) {
                if map_size.is_inside_map(neighbor_x, neighbor_y) {
                    result.push((neighbor_x as usize, neighbor_y as usize));
                }
            }
        }

        result
    }

    let mut result = DMatrix::from_element(
        map_size.total_size() as usize,
        map_size.total_size() as usize,
        true,
    );

    for x in 0..map_size.total_size() {
        for y in 0..map_size.total_size() {
            if buildings_grid.0[(x as usize, y as usize)].is_some() {
                for neighbor in get_neighbors(&map_size, x as i32, y as i32) {
                    result[neighbor] = false;
                }
            }
        }
    }

    drop_zone.set(DropZone(result)).unwrap();
}
