use nalgebra::Vector2;

use super::{
    Segment,
    Square,
};

pub struct Ray {
    pub start: Vector2<f32>,
    pub direction: Vector2<f32>,
}

impl Ray {
    /// angle in radians
    pub fn new_with_angle(start: Vector2<f32>, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self {
            start,
            direction: Vector2::new(cos, sin),
        }
    }

    /// https://stackoverflow.com/questions/14307158/how-do-you-check-for-intersection-between-a-line-segment-and-a-line-ray-emanatin
    pub fn intersection_with_segment(&self, segment: &Segment) -> Option<Vector2<f32>> {
        let p = self.start;
        let r = self.direction;

        let q = segment.a;
        let s = segment.b - segment.a;

        if cross_2d(r, s).abs() < 0.00001 {
            return None;
        }

        let t = cross_2d(q - p, s / cross_2d(r, s));
        let u = cross_2d(q - p, r / cross_2d(r, s));

        if t >= 0.0 && (0.0..=1.0).contains(&u) {
            Some(q + s * u)
        } else {
            None
        }
    }

    pub fn intersection_with_square(&self, square: &Square) -> Option<Vector2<f32>> {
        for segment in &square.segments {
            let intersection = self.intersection_with_segment(segment);

            if intersection.is_some() {
                return intersection;
            }
        }

        None
    }
}

fn cross_2d(lhs: Vector2<f32>, rhs: Vector2<f32>) -> f32 {
    lhs.x * rhs.y - lhs.y * rhs.x
}
