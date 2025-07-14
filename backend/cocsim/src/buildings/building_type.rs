use nalgebra::Vector2;
use serde::Serialize;

use crate::{
    BuildingOption,
    Game,
    Shape,
};

#[derive(Serialize)]
pub struct BuildingType {
    name: String,
    size: Vector2<u32>,
    levels: u32,
    options: Vec<BuildingOption>,

    #[serde(skip)]
    tick_fn: Option<fn(game: &mut Game, id: u32, delta_t: f32)>,
    #[serde(skip)]
    draw_fn: Option<fn(game: &Game, id: u32, result: &mut Vec<Shape>)>,
}

impl BuildingType {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> Vector2<u32> {
        self.size
    }

    pub fn levels(&self) -> u32 {
        self.levels
    }

    pub fn options(&self) -> &[BuildingOption] {
        &self.options
    }

    pub fn tick_fn(&self) -> Option<fn(game: &mut Game, id: u32, delta_t: f32)> {
        self.tick_fn
    }

    pub fn draw_fn(&self) -> Option<fn(game: &Game, id: u32, result: &mut Vec<Shape>)> {
        self.draw_fn
    }
}
