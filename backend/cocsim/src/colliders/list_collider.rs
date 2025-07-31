use nalgebra::Vector2;
use rand::distr::{
    Distribution,
    weighted::WeightedIndex,
};

use super::{
    Collider,
    ColliderEnum,
};

#[derive(Debug, Clone)]
pub struct ListCollider {
    colliders: Vec<ColliderEnum>,
}

impl ListCollider {
    pub fn new(colliders: Vec<ColliderEnum>) -> Self {
        Self { colliders }
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

    fn translate(&self, offset: Vector2<f32>) -> Self {
        Self {
            colliders: self
                .colliders
                .iter()
                .map(|collider| collider.translate(offset))
                .collect(),
        }
    }

    fn area(&self) -> f32 {
        self.colliders().iter().map(ColliderEnum::area).sum()
    }

    fn random_point(&self, rng: &mut impl rand::Rng) -> Vector2<f32> {
        let dist = WeightedIndex::new(self.colliders.iter().map(ColliderEnum::area)).unwrap();

        self.colliders[dist.sample(rng)].random_point(rng)
    }

    fn contains(&self, point: Vector2<f32>) -> bool {
        self.colliders
            .iter()
            .any(|collider| collider.contains(point))
    }
}
