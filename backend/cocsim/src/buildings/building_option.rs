use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct BuildingOption {
    pub name: String,
    pub values: Vec<String>,
}
