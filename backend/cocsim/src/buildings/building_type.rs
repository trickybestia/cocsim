use nalgebra::Vector2;
use serde::Serialize;

use crate::BuildingOption;

#[derive(Serialize)]
pub struct BuildingType {
    pub name: &'static str,
    pub size: Vector2<usize>,
    pub levels: usize,
    pub options: Vec<BuildingOption>,
}
