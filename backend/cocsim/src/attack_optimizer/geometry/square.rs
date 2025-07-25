use nalgebra::Vector2;

use super::Segment;

pub struct Square {
    pub segments: [Segment; 4],
}

impl Square {
    pub fn new_from_center(center: Vector2<f32>, side: f32) -> Self {
        let left_bottom = center - Vector2::from_element(side) / 2.0;
        let left_top = left_bottom + Vector2::new(0.0, side);
        let right_bottom = left_bottom + Vector2::new(side, 0.0);
        let right_top = right_bottom + Vector2::new(0.0, side);

        Self {
            segments: [
                Segment::new(left_top, right_top),
                Segment::new(right_top, right_bottom),
                Segment::new(right_bottom, left_bottom),
                Segment::new(left_bottom, left_top),
            ],
        }
    }
}
