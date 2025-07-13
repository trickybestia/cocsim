use std::ops::Deref;

use nalgebra::{
    DMatrix,
    Vector2,
};

use crate::{
    BuildingType,
    Game,
    colliders::Collider,
};

pub trait Building
where
    Self: Deref<Target = &'static BuildingType>,
{
    fn position(&self) -> Vector2<u32>;
    fn health(&self) -> f32;
    fn collider(&self) -> Option<&dyn Collider>;

    /// Returns [`Vec`] of on_destroyed event handlers.
    fn on_destroyed_mut(&mut self) -> Vec<Box<dyn Fn(&mut Game, u32)>>;

    fn center(&self) -> Vector2<f32> {
        self.position().cast() + self.size().cast() / 2.0
    }

    fn destroyed(&self) -> bool {
        self.health() == 0.0
    }

    /// Apply damage to this building. Called by units when they attack.
    fn apply_damage(&mut self, damage: f32);

    /// Occupy tiles for troops drop zone calculation. Called once.
    fn occupy_tiles(&self, id: u32, buildings_grid: &mut DMatrix<Option<u32>>) {
        for x in (self.position().x)..(self.position().x + self.size().x) {
            for y in (self.position().y)..(self.position().y + self.size().y) {
                buildings_grid[(x as usize, y as usize)] = Some(id)
            }
        }
    }

    /// Update collision for this building. Can be called multiple times. Need
    /// check for self.destroyed.
    fn update_collision(&self, id: u32, collision_grid: &mut DMatrix<Option<u32>>);
}
