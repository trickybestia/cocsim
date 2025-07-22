mod dragon;
pub mod utils;

use anyhow::Result;
pub use dragon::*;
use enum_dispatch::enum_dispatch;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};
use shipyard::World;

#[enum_dispatch]
pub trait UnitModel {
    fn create_unit(&self, world: &mut World, position: Vector2<f32>) -> Result<()>;
}

#[derive(Serialize)]
pub struct UnitType {
    pub name: &'static str,
    pub housing_space: usize,
    pub levels: usize,
}

#[enum_dispatch(UnitModel)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name")]
pub enum UnitModelEnum {
    #[serde(rename = "Dragon")]
    DragonModel,
}
