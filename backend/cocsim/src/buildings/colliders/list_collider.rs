use nalgebra::Vector2;

use super::Collider;
use crate::colliders::ColliderEnum;

pub struct ListCollider {
    colliders: Vec<ColliderEnum>,
}

impl ListCollider {
    pub fn new<T>(colliders: T) -> Self
    where
        T: Into<Vec<ColliderEnum>>,
    {
        Self {
            colliders: colliders.into(),
        }
    }
}

impl Collider for ListCollider {
    fn attack_area(&self, attack_range: f32) -> ColliderEnum {
        ColliderEnum::ListCollider(ListCollider::new(
            self.colliders
                .iter()
                .map(|collider| collider.attack_area(attack_range))
                .collect::<Vec<_>>(),
        ))
    }

    fn nearest_point(&self, point: Vector2<f32>) -> Vector2<f32> {
        let mut best_nearest_point = self.colliders[0].nearest_point(point);
        let mut best_nearest_point_distance = point.metric_distance(&best_nearest_point);

        for collider in self.colliders.iter().skip(1) {
            let current_nearest_point = collider.nearest_point(point);
            let current_nearest_point_distance = point.metric_distance(&current_nearest_point);

            if current_nearest_point_distance < best_nearest_point_distance {
                best_nearest_point = current_nearest_point;
                best_nearest_point_distance = current_nearest_point_distance;
            }
        }

        best_nearest_point
    }

    fn contains(&self, point: Vector2<f32>) -> bool {
        self.colliders
            .iter()
            .any(|collider| collider.contains(point))
    }
}
