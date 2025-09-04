use std::f32::{
    INFINITY,
    NEG_INFINITY,
    consts::PI,
};

use nalgebra::{
    DMatrix,
    Vector2,
    clamp,
};

use crate::{
    consts::MAX_UNIT_DROP_TIME,
    game::features::map_size::MapSize,
    geometry::{
        Ray,
        Rect,
        Segment,
    },
};

#[derive(Clone, Debug)]
pub struct AttackPlanUnitGroup {
    /// radians
    pub angle: f32,
    /// from 0 to 1
    pub distance: f32,
    pub drop_time: f32,
}

impl AttackPlanUnitGroup {
    pub fn from_numbers(x: &mut impl Iterator<Item = f32>) -> Self {
        Self {
            angle: x.next().unwrap(),
            distance: x.next().unwrap(),
            drop_time: x.next().unwrap(),
        }
    }

    pub fn domain() -> impl Iterator<Item = (f32, f32)> {
        [
            (NEG_INFINITY, INFINITY),  // angle
            (0.0, 1.0),                // distance
            (0.0, MAX_UNIT_DROP_TIME), // drop_time
        ]
        .into_iter()
    }

    pub fn cartesian_position(
        &self,
        map_size: &MapSize,
        drop_zone: &DMatrix<bool>,
    ) -> Vector2<f32> {
        let offset = map_size.total_size() as f32 / 2.0;
        let center = Vector2::from_element(offset);

        let ray = Ray::new_with_angle(center, self.angle);
        let border_square = Rect::new_square_from_center(center, map_size.total_size() as f32);
        let mut start_point = None;
        let stop_point = ray.intersection_with_rect(&border_square).unwrap();
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

        // if there is no intersected buildings, we can place unit anywhere on the line
        let start_point = start_point.unwrap_or(
            Ray::new_with_angle(center, self.angle + PI)
                .intersection_with_rect(&border_square)
                .unwrap(),
        );

        start_point + (stop_point - start_point) * clamp(self.distance, 0.01, 0.99) // clamp for unit to not spawn on right or bottom border
    }
}
