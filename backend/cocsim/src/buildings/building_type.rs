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

    #[serde(skip)]
    pub tick_fn: Option<fn(game: &mut Game, id: u32, delta_t: f32)>,
    #[serde(skip)]
    pub draw_fn: Option<fn(game: &Game, id: u32, result: &mut Vec<Shape>)>,
}
