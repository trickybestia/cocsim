use nalgebra::Vector2;

use super::{
    Collider,
    ColliderEnum,
    RectCollider,
};

pub struct ListCollider {
    colliders: Vec<ColliderEnum>,
    bounding_box: RectCollider,
}

impl ListCollider {
    pub fn new(colliders: Vec<ColliderEnum>) -> Self {
        let mut left_x = colliders[0].bounding_box().position.x;
        let mut right_x = left_x + colliders[0].bounding_box().size.x;
        let mut top_y = colliders[0].bounding_box().position.y;
        let mut bottom_y = top_y + colliders[0].bounding_box().size.y;

        for collider in &colliders[1..] {
            let collider_bounding_box = collider.bounding_box();

            left_x = left_x.min(collider_bounding_box.position.x);
            right_x = right_x.max(collider_bounding_box.position.x + collider_bounding_box.size.x);
            top_y = top_y.min(collider_bounding_box.position.y);
            bottom_y =
                bottom_y.max(collider_bounding_box.position.y + collider_bounding_box.size.y);
        }

        Self {
            colliders,
            bounding_box: RectCollider::new(
                Vector2::new(left_x, top_y),
                Vector2::new(right_x - left_x, bottom_y - top_y),
            ),
        }
    }

    pub fn colliders(&self) -> &[ColliderEnum] {
        &self.colliders
    }
}

impl Collider for ListCollider {
    fn attack_area(&self, attack_range: f32) -> ColliderEnum {
        ListCollider::new(
            self.colliders
                .iter()
                .map(|collider| collider.attack_area(attack_range))
                .collect::<Vec<_>>(),
        )
        .into()
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

    fn bounding_box(&self) -> RectCollider {
        self.bounding_box.clone()
    }

    fn contains(&self, point: Vector2<f32>) -> bool {
        self.colliders
            .iter()
            .any(|collider| collider.contains(point))
    }
}
