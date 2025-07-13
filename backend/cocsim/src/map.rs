use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct Map {
    pub base_size: usize,
    pub border_size: usize,
}
