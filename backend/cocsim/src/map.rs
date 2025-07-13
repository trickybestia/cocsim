use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct Map {
    pub base_size: u32,
    pub border_size: u32,
}
