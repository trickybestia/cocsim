use serde::Deserialize;

use crate::BuildingModelEnum;

#[derive(Deserialize)]
pub struct Map {
    pub base_size: usize,
    pub border_size: usize,

    pub buildings: Vec<BuildingModelEnum>,
}
