use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};

use crate::BuildingOption;

#[derive(Serialize, Deserialize)]
pub struct BuildingType {
    pub name: &'static str,
    pub size: Vector2<usize>,
    pub levels: usize,
    pub options: Vec<BuildingOption>,
}
