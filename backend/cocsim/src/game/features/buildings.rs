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
