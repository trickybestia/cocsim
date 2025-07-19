use enum_dispatch::enum_dispatch;
use nalgebra::Vector2;

use super::{
    ColliderEnum,
    RectCollider,
};

#[enum_dispatch]
pub trait Collider {
    /// Returns attack area [`Collider`] from which units with given
    /// `attack_range` can attack this building.
    fn attack_area(&self, attack_range: f32) -> ColliderEnum;

    /// Returns point of collider nearest to `point`.
    fn nearest_point(&self, point: Vector2<f32>) -> Vector2<f32>;

    fn bounding_box(&self) -> RectCollider;

    /// Checks if `point` is inside of collider.
    fn contains(&self, point: Vector2<f32>) -> bool;
}
