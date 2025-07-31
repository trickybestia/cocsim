use enum_dispatch::enum_dispatch;
use nalgebra::Vector2;
use rand::Rng;

use super::ColliderEnum;

#[enum_dispatch]
pub trait Collider
where
    Self: Clone,
{
    /// Returns attack area [`Collider`] from which units with given
    /// `attack_range` can attack this building.
    fn attack_area(&self, attack_range: f32) -> ColliderEnum;

    /// Returns point of collider nearest to `point`.
    fn nearest_point(&self, point: Vector2<f32>) -> Vector2<f32>;

    fn translate(&self, offset: Vector2<f32>) -> Self;

    fn area(&self) -> f32;

    fn random_point(&self, rng: &mut impl Rng) -> Vector2<f32>;

    /// Checks if `point` is inside of collider.
    fn contains(&self, point: Vector2<f32>) -> bool;
}
