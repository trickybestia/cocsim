use nalgebra::Vector2;

#[derive(Debug)]
pub struct Segment {
    pub a: Vector2<f32>,
    pub b: Vector2<f32>,
}

impl Segment {
    pub fn new(a: Vector2<f32>, b: Vector2<f32>) -> Self {
        Self { a, b }
    }
}
