use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct BuildingOption {
    name: String,
    values: Vec<String>,
}

impl BuildingOption {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn values(&self) -> &[String] {
        &self.values
    }
}
