use std::f32::consts::PI;

use nalgebra::Vector2;
use rand::Rng;

use super::{
    Collider,
    ColliderEnum,
};
use crate::geometry::{
    Ray,
    Rect,
};

#[derive(Debug, Clone)]
pub struct RectCollider {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
}

impl RectCollider {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> Self {
        Self { position, size }
    }

    pub fn new_from_center(center: Vector2<f32>, size: Vector2<f32>) -> Self {
        Self::new(center - size / 2.0, size)
    }
}

impl Collider for RectCollider {
    fn attack_area(&self, attack_range: f32) -> ColliderEnum {
        ColliderEnum::RectCollider(RectCollider {
            position: self.position - Vector2::from_element(attack_range),
            size: self.size + Vector2::from_element(attack_range * 2.0),
        })
    }

    fn nearest_point(&self, point: Vector2<f32>) -> Vector2<f32> {
        if self.contains(point) {
            point
        } else {
            if self.position.x <= point.x && point.x <= self.position.x + self.size.x {
                if point.y <= self.position.y {
                    Vector2::new(point.x, self.position.y)
                } else {
                    Vector2::new(point.x, self.position.y + self.size.y)
                }
            } else if self.position.y <= point.y && point.y <= self.position.y + self.size.y {
                if point.x <= self.position.x {
                    Vector2::new(self.position.x, point.y)
                } else {
                    Vector2::new(self.position.x + self.size.x, point.y)
                }
            } else if point.x <= self.position.x {
                if point.y <= self.position.y {
                    Vector2::new(self.position.x, self.position.y)
                } else {
                    Vector2::new(self.position.x, self.position.y + self.size.y)
                }
            } else {
                if point.y <= self.position.y {
                    Vector2::new(self.position.x + self.size.x, self.position.y)
                } else {
                    Vector2::new(self.position.x + self.size.x, self.position.y + self.size.y)
                }
            }
        }
    }

    fn translate(&self, offset: Vector2<f32>) -> Self {
        Self::new(self.position + offset, self.size)
    }

    fn random_near_point(&self, point: Vector2<f32>, rng: &mut impl Rng) -> Vector2<f32> {
        if self.contains(point) {
            return point;
        }

        let rect = Rect::new(self.position, self.size);

        let mut angles = rect.vertices.map(|vertex| {
            Ray {
                start: point,
                direction: (vertex - point).normalize(),
            }
            .angle()
        });

        angles.sort_unstable_by(|a, b| a.total_cmp(&b));

        const STEPS_COUNT: usize = 100;

        let start_angle: f32;
        let step: f32;

        if angles[3] - angles[0] < PI {
            start_angle = angles[0];
            step = (angles[3] - start_angle) / STEPS_COUNT as f32;
        } else {
            start_angle = angles[3];
            step = (angles[0] + 2.0 * PI - start_angle) / STEPS_COUNT as f32;
        }

        let step_index = rng.random_range(1..STEPS_COUNT); // avoid using boundaries (0 and STEPS_COUNT)

        Ray::new_with_angle(point, start_angle + step * step_index as f32)
            .intersection_with_rect(&rect)
            .expect("Expected at least one intersection")
    }

    fn contains(&self, point: Vector2<f32>) -> bool {
        self.position <= point && point <= self.position + self.size
    }
}
