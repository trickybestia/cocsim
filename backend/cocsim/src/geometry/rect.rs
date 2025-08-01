use nalgebra::Vector2;

use super::Segment;

#[derive(Debug)]
pub struct Rect {
    pub segments: [Segment; 4],
    pub vertices: [Vector2<f32>; 4],
}

impl Rect {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> Self {
        let left_bottom = position;
        let left_top = position + Vector2::new(0.0, size.y);
        let right_bottom = position + Vector2::new(size.x, 0.0);
        let right_top = position + size;

        Self {
            segments: [
                Segment::new(left_top, right_top),
                Segment::new(right_top, right_bottom),
                Segment::new(right_bottom, left_bottom),
                Segment::new(left_bottom, left_top),
            ],
            vertices: [left_bottom, left_top, right_top, right_bottom],
        }
    }

    pub fn new_from_center(center: Vector2<f32>, size: Vector2<f32>) -> Self {
        Self::new(center - size / 2.0, size)
    }

    pub fn new_square_from_center(center: Vector2<f32>, side: f32) -> Self {
        Self::new_from_center(center, Vector2::from_element(side))
    }
}
