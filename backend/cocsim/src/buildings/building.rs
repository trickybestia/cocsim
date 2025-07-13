use std::ops::Deref;

use nalgebra::Vector2;

use crate::{
    BuildingType,
    Game,
    Shape,
    colliders::Collider,
};

pub trait Building
where
    Self: Deref<Target = BuildingType>,
{
    fn id(&self) -> u32;

    fn position(&self) -> Vector2<u32>;
    fn health(&self) -> f32;
    fn collider(&self) -> Option<&dyn Collider>;

    /// Returns [`Vec`] of on_destroyed event handlers.
    fn on_destroyed_mut(&mut self) -> Vec<Box<dyn Fn(&dyn Building)>>;

    fn center(&self) -> Vector2<f32> {
        self.position().cast() + self.size().cast() / 2.0
    }

    fn destroyed(&self) -> bool {
        self.health() == 0.0
    }

    fn tick(&mut self, game: &mut Game, delta_t: f32);

    fn draw(&self, game: &Game, shapes: &mut Vec<Shape>);

    /// Apply damage to this building. Called by units when they attack.
    fn apply_damage(&mut self, damage: f32);

    /// Occupy tiles for troops drop zone calculation. Called once.
    fn occupy_tiles(&self, game: &mut Game) {
        for x in (self.position().x)..(self.position().x + self.size().x) {
            for y in (self.position().y)..(self.position().y + self.size().y) {
                todo!()
            }
        }
    }

    /// Update collision for this building. Can be called multiple times. Need
    /// check for self.destroyed.
    fn update_collision(&self, game: &mut Game);
}
