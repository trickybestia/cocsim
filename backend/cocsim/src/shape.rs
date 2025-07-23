use std::fmt::Display;

use serde::{
    Serialize,
    ser::SerializeTuple,
};
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

impl Serialize for Shape {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Shape::Rect {
                x,
                y,
                width,
                height,
                color,
            } => {
                let mut tuple = serializer.serialize_tuple(6)?;

                tuple.serialize_element("rect")?;
                tuple.serialize_element(x)?;
                tuple.serialize_element(y)?;
                tuple.serialize_element(width)?;
                tuple.serialize_element(height)?;
                tuple.serialize_element(color)?;

                tuple.end()
            }
            Shape::Circle {
                x,
                y,
                radius,
                color,
            } => {
                let mut tuple = serializer.serialize_tuple(4)?;

                tuple.serialize_element("circle")?;
                tuple.serialize_element(x)?;
                tuple.serialize_element(y)?;
                tuple.serialize_element(radius)?;
                tuple.serialize_element(color)?;

                tuple.end()
            }
            Shape::Line {
                x1,
                y1,
                x2,
                y2,
                width,
                color,
            } => {
                let mut tuple = serializer.serialize_tuple(7)?;

                tuple.serialize_element("line")?;
                tuple.serialize_element(x1)?;
                tuple.serialize_element(y1)?;
                tuple.serialize_element(x2)?;
                tuple.serialize_element(y2)?;
                tuple.serialize_element(width)?;
                tuple.serialize_element(color)?;

                tuple.end()
            }
        }
    }
}
