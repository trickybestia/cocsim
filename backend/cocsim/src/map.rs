use serde::{
    Deserialize,
    Serialize,
};

use crate::BuildingModelEnum;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    pub base_size: usize,
    pub border_size: usize,

    pub buildings: Vec<BuildingModelEnum>,
}
