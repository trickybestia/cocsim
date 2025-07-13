use nalgebra::Vector2;

pub trait Building {
    fn position(&self) -> Vector2<f32>;
    fn health(&self) -> f32;
}
