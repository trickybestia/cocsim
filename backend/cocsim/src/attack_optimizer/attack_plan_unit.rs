use std::f32::consts::PI;

use nalgebra::{
    DMatrix,
    Vector2,
    clamp,
};
use rand::Rng;

use crate::{
    UnitModelEnum,
    attack_optimizer::geometry::{
        Ray,
        Segment,
        Square,
    },
    consts::MAX_UNIT_DROP_TIME,
    game::features::map::MapSize,
};

#[derive(Clone)]
pub struct AttackPlanUnit {
    unit_model: UnitModelEnum,
    /// radians
    angle: f32,
    /// from 0 to 1
    distance: f32,
    drop_time: f32,
}

impl AttackPlanUnit {
    pub fn new_randomized(unit_model: UnitModelEnum, rng: &mut impl Rng) -> Self {
        Self {
            unit_model,
            angle: rng.random_range(0.0..(2.0 * PI)),
            distance: rng.random_range(0.0..=1.0),
            drop_time: rng.random_range(0.0..=MAX_UNIT_DROP_TIME),
        }
    }

    pub fn mutate(&self, rng: &mut impl Rng) -> Self {
        Self {
            unit_model: self.unit_model.clone(),
            angle: self.angle + rng.random_range((-1.0)..=1.0),
            distance: clamp(self.distance + rng.random_range((-0.2)..=0.2), 0.0, 1.0),
            drop_time: clamp(
                self.drop_time + rng.random_range((-0.5)..=0.5),
                0.0,
                MAX_UNIT_DROP_TIME,
            ),
        }
    }

    pub fn cartesian_position(
        &self,
        map_size: &MapSize,
        drop_zone: &DMatrix<bool>,
    ) -> Vector2<f32> {
        let offset = map_size.total_size() as f32 / 2.0;
        let center = Vector2::from_element(offset);

        let ray = Ray::new_with_angle(center, self.angle);
        let border_square = Square::new_from_center(center, map_size.total_size() as f32);
        let mut start_point = None;
        let stop_point = ray.intersection_with_square(&border_square).unwrap();
        let segment_length = (stop_point - ray.start).norm();

        for t in 0..100 {
            let distance = segment_length * t as f32 / 100.0;
            let tile_pos = stop_point - ray.direction * distance;
            let x_i32 = clamp(tile_pos.x as i32, 0, map_size.total_size() - 1);
            let y_i32 = clamp(tile_pos.y as i32, 0, map_size.total_size() - 1);
            let x_usize = x_i32 as usize;
            let y_usize = y_i32 as usize;

            if map_size.is_inside_map(Vector2::new(x_i32 + 1, y_i32))
                && drop_zone[(x_usize, y_usize)] != drop_zone[(x_usize + 1, y_usize)]
            // right tile border
            {
                start_point = ray.intersection_with_segment(&Segment::new(
                    Vector2::new(x_usize + 1, y_usize).cast(),
                    Vector2::new(x_usize + 1, y_usize + 1).cast(),
                ));

                if start_point.is_some() {
                    break;
                }
            }

            if map_size.is_inside_map(Vector2::new(x_i32, y_i32 + 1))
                && drop_zone[(x_usize, y_usize)] != drop_zone[(x_usize, y_usize + 1)]
            // down tile border
            {
                start_point = ray.intersection_with_segment(&Segment::new(
                    Vector2::new(x_usize, y_usize + 1).cast(),
                    Vector2::new(x_usize + 1, y_usize + 1).cast(),
                ));

                if start_point.is_some() {
                    break;
                }
            }

            if map_size.is_inside_map(Vector2::new(x_i32 - 1, y_i32))
                && drop_zone[(x_usize, y_usize)] != drop_zone[(x_usize - 1, y_usize)]
            // left tile border
            {
                start_point = ray.intersection_with_segment(&Segment::new(
                    Vector2::new(x_usize, y_usize).cast(),
                    Vector2::new(x_usize, y_usize + 1).cast(),
                ));

                if start_point.is_some() {
                    break;
                }
            }

            if map_size.is_inside_map(Vector2::new(x_i32, y_i32 - 1))
                && drop_zone[(x_usize, y_usize)] != drop_zone[(x_usize, y_usize - 1)]
            // up tile border
            {
                start_point = ray.intersection_with_segment(&Segment::new(
                    Vector2::new(x_usize, y_usize).cast(),
                    Vector2::new(x_usize + 1, y_usize).cast(),
                ));

                if start_point.is_some() {
                    break;
                }
            }
        }

        let start_point = start_point.expect("At least one intersection expected");

        let result = start_point + (stop_point - start_point) * clamp(self.distance, 0.01, 0.99); // clamp for unit to not spawn on right or bottom border

        result
    }

    pub fn unit_model(&self) -> &UnitModelEnum {
        &self.unit_model
    }

    pub fn drop_time(&self) -> f32 {
        self.drop_time
    }
}
