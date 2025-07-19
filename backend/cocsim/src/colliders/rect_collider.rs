use nalgebra::Vector2;

use super::{
    Collider,
    ColliderEnum,
};

#[derive(Clone)]
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

    fn bounding_box(&self) -> &Self {
        self
    }

    fn contains(&self, point: Vector2<f32>) -> bool {
        self.position <= point && point <= self.position + self.size
    }
}
