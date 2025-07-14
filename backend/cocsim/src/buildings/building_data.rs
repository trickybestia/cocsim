use std::ops::Deref;

use nalgebra::{
    DMatrix,
    Vector2,
};

use crate::{
    BuildingType,
    Game,
    colliders::Collider,
    consts::*,
};

pub trait BuildingData
where
    Self: Deref<Target = &'static BuildingType>,
{
    fn position(&self) -> Vector2<usize>;
    fn health(&self) -> f32;
    fn collider(&self) -> Option<&dyn Collider>;

    /// Returns [`Vec`] of on_destroyed event handlers.
    fn on_destroyed_mut(&mut self) -> &mut Vec<Box<dyn Fn(&mut Game, usize)>>;

    fn center(&self) -> Vector2<f32> {
        self.position().cast() + self.size.cast() / 2.0
    }

    fn destroyed(&self) -> bool {
        self.health() == 0.0
    }

    /// Apply damage to this building. Called by units when they attack.
    fn apply_damage(&mut self, damage: f32);

    /// Occupy tiles for troops drop zone calculation. Called once.
    fn occupy_tiles(&self, id: usize, buildings_grid: &mut DMatrix<Option<usize>>) {
        for rel_x in 0..self.size.x {
            let abs_x = self.position().x + rel_x;

            for rel_y in 0..self.size.y {
                let abs_y = self.position().y + rel_y;

                buildings_grid[(abs_x, abs_y)] = Some(id)
            }
        }
    }

    /// Update collision for this building. Can be called multiple times. Need
    /// check for self.destroyed.
    fn update_collision(&self, id: usize, collision_grid: &mut DMatrix<Option<usize>>) {
        if let Some(collider) = self.collider() {
            for rel_x in 0..(self.size.x * COLLISION_TILES_PER_MAP_TILE) {
                let abs_x = self.position().x * COLLISION_TILES_PER_MAP_TILE + rel_x;

                for rel_y in 0..self.size.y {
                    let abs_y = self.position().y * COLLISION_TILES_PER_MAP_TILE + rel_y;

                    let occupy_tile = !self.destroyed()
                        && collider.contains(Vector2::new(
                            abs_x as f32 / COLLISION_TILES_PER_MAP_TILE as f32,
                            abs_y as f32 / COLLISION_TILES_PER_MAP_TILE as f32,
                        ));

                    collision_grid[(abs_x, abs_y)] = if occupy_tile { Some(id) } else { None }
                }
            }
        }
    }
}
