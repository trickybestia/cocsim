use nalgebra::Vector2;
use serde::Serialize;

use crate::{
    BuildingOption,
    Game,
    Shape,
};

#[derive(Serialize)]
pub struct BuildingType {
    pub name: &'static str,
    pub size: Vector2<usize>,
    pub levels: usize,
    pub options: Vec<BuildingOption>,
}

impl BuildingType {
    pub fn tick(&self) -> Option<fn(game: &mut Game, id: usize, delta_t: f32)> {
        None
    }

    pub fn draw(&self) -> Option<fn(game: &Game, id: usize, result: &mut Vec<Shape>)> {
        None
    }
}
