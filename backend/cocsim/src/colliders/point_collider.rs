use nalgebra::Vector2;
use rand::Rng;

use super::{
    Collider,
    ColliderEnum,
};
use crate::colliders::CircleCollider;

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
        CircleCollider::new(self.0, attack_range).into()
    }

    fn nearest_point(&self, _point: Vector2<f32>) -> Vector2<f32> {
        self.0
    }

    fn translate(&self, offset: Vector2<f32>) -> Self {
        Self::new(self.0 + offset)
    }

    fn random_near_point(&self, _point: Vector2<f32>, _rng: &mut impl Rng) -> Vector2<f32> {
        self.0
    }

    fn contains(&self, point: Vector2<f32>) -> bool {
        self.0 == point
    }
}
