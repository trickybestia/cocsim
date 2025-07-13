use nalgebra::Vector2;

pub trait Collider {
    /// Returns attack area `Collider` from which units with given
    /// `attack_range` can attack this building.
    fn attack_area(&self, attack_range: f32) -> Box<dyn Collider>;

    /// Returns point of collider nearest to `point`.
    fn nearest_point(&self, point: Vector2<f32>) -> Vector2<f32>;

    /// Checks if `point` is inside of collider.
    fn contains(&self, point: Vector2<f32>) -> bool;
}
