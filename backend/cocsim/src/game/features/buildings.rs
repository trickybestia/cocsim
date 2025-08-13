use hecs::{
    Entity,
    World,
};
use nalgebra::{
    DMatrix,
    Vector2,
};

use crate::game::features::map_size::MapSize;

pub struct Building {
    pub position: Vector2<usize>,
    pub size: Vector2<usize>,
    /// `true` for most buildings, `false` for traps.
    pub affects_drop_zone: bool,
    /// `true` for most buildings, `false` for walls and traps.
    pub affects_percentage_destroyed: bool,
}

pub struct TownHall;

pub struct BuildingsGrid(pub DMatrix<Entity>);

impl BuildingsGrid {
    pub fn new(map_size: &MapSize, world: &mut World) -> Self {
        let mut result = DMatrix::from_element(
            map_size.total_size() as usize,
            map_size.total_size() as usize,
            Entity::DANGLING,
        );

        for (id, building) in world.query_mut::<&Building>() {
            if !building.affects_drop_zone {
                continue;
            }

            for rel_x in 0..building.size.x {
                let abs_x = building.position.x + rel_x;

                for rel_y in 0..building.size.y {
                    let abs_y = building.position.y + rel_y;

                    result[(abs_x, abs_y)] = id;
                }
            }
        }

        Self(result)
    }
}

pub struct DropZone(pub DMatrix<bool>);

impl DropZone {
    pub fn new(map_size: &MapSize, buildings_grid: &BuildingsGrid) -> Self {
        fn get_neighbors(map_size: &MapSize, x: i32, y: i32) -> Vec<(usize, usize)> {
            let mut result = Vec::new();

            for neighbor_x in (x - 1)..(x + 2) {
                for neighbor_y in (y - 1)..(y + 2) {
                    if map_size.is_inside_map(Vector2::new(neighbor_x, neighbor_y)) {
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
                if buildings_grid.0[(x as usize, y as usize)] != Entity::DANGLING {
                    for neighbor in get_neighbors(&map_size, x, y) {
                        result[neighbor] = false;
                    }
                }
            }
        }

        Self(result)
    }
}
