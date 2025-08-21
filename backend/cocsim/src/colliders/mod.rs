mod circle_collider;
mod list_collider;
mod point_collider;
mod rect_collider;

pub use circle_collider::CircleCollider;
use enum_dispatch::enum_dispatch;
pub use list_collider::ListCollider;
use nalgebra::Vector2;
pub use point_collider::PointCollider;
use rand::Rng;
pub use rect_collider::RectCollider;

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

    /// Returns random viewable point from `point` point of view. This is used
    /// by air units targeting.
    fn random_near_point(&self, point: Vector2<f32>, rng: &mut impl Rng) -> Vector2<f32>;

    /// Checks if `point` is inside of collider.
    fn contains(&self, point: Vector2<f32>) -> bool;
}

#[enum_dispatch(Collider)]
#[derive(Debug, Clone)]
pub enum ColliderEnum {
    CircleCollider,
    ListCollider,
    RectCollider,
    PointCollider,
}
