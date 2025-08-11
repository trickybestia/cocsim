use std::f32::consts::FRAC_PI_2;

use nalgebra::Vector2;
use rand::Rng;

use super::{
    Collider,
    ColliderEnum,
};

#[derive(Debug, Clone)]
pub struct CircleCollider {
    pub center: Vector2<f32>,
    pub radius: f32,
}

impl CircleCollider {
    pub fn new(center: Vector2<f32>, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Collider for CircleCollider {
    fn attack_area(&self, attack_range: f32) -> ColliderEnum {
        Self::new(self.center, self.radius + attack_range).into()
    }

    fn nearest_point(&self, point: Vector2<f32>) -> Vector2<f32> {
        self.center + (point - self.center).normalize() * self.radius
    }

    fn translate(&self, offset: Vector2<f32>) -> Self {
        Self::new(self.center + offset, self.radius)
    }

    fn random_near_point(&self, point: Vector2<f32>, rng: &mut impl Rng) -> Vector2<f32> {
        if self.contains(point) {
            return point;
        }

        let offset = point - self.center;
        let angle = offset.y.atan2(offset.x);

        let random_angle_start = angle - FRAC_PI_2;
        let random_angle_end = angle + FRAC_PI_2;
        let random_angle = rng.random_range(random_angle_start..=random_angle_end);

        self.center + Vector2::new(random_angle.sin(), random_angle.cos()) * self.radius
    }

    fn contains(&self, point: Vector2<f32>) -> bool {
        self.center.metric_distance(&point) <= self.radius
    }
}
