use std::fmt::Display;

use serde::Serialize;
use serde_with::SerializeDisplay;

#[derive(Clone, Copy, SerializeDisplay)]
pub struct ShapeColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Display for ShapeColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl ShapeColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Serialize)]
pub enum Shape {
    Rect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: ShapeColor,
    },
    Circle {
        x: f32,
        y: f32,
        radius: f32,
        color: ShapeColor,
    },
    Line {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        width: f32,
        color: ShapeColor,
    },
}
