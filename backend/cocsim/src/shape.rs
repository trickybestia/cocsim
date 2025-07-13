use std::borrow::Cow;

pub type ShapeColor = Cow<'static, str>;

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
