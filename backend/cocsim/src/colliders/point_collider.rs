use nalgebra::Vector2;

use super::{
    Collider,
    ColliderEnum,
};
use crate::colliders::RectCollider;

#[derive(Debug, Clone)]
pub struct PointCollider(pub Vector2<f32>);

impl PointCollider {
    pub fn new(position: Vector2<f32>) -> Self {
        Self(position)
    }

    pub fn zero() -> Self {
        Self(Vector2::from_element(0.0))
    }
}

impl Collider for PointCollider {
    fn attack_area(&self, attack_range: f32) -> ColliderEnum {
        ColliderEnum::RectCollider(RectCollider {
            position: self.0 - Vector2::from_element(attack_range),
            size: Vector2::from_element(attack_range * 2.0),
        })
    }

    fn nearest_point(&self, _point: Vector2<f32>) -> Vector2<f32> {
        self.0
    }

    fn translate(&self, offset: Vector2<f32>) -> Self {
        Self::new(self.0 + offset)
    }

    fn bounding_box(&self) -> RectCollider {
        RectCollider::new(self.0, Vector2::zeros())
    }

    fn contains(&self, point: Vector2<f32>) -> bool {
        self.0 == point
    }
}
