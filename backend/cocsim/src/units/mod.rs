mod balloon;
mod dragon;
pub mod utils;

use anyhow::ensure;
use arbitrary::Arbitrary;
pub use balloon::*;
pub use dragon::*;
use enum_dispatch::enum_dispatch;
use hecs::World;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    consts::MAX_UNITS_COUNT,
    game::features::attack::Team,
};

#[enum_dispatch]
pub trait UnitModel {
    fn spawn(&self, world: &mut World, position: Vector2<f32>, team: Team);
}

#[derive(Serialize)]
pub struct UnitType {
    pub name: &'static str,
    pub housing_space: usize,
    pub levels: usize,
}

inventory::collect!(UnitType);

#[enum_dispatch(UnitModel)]
#[derive(Serialize, Deserialize, Debug, Clone, Arbitrary)]
#[serde(tag = "name")]
pub enum UnitModelEnum {
    #[serde(rename = "Balloon")]
    BalloonModel,
    #[serde(rename = "Dragon")]
    DragonModel,
}

pub fn validate_units<'a, T>(units: T) -> anyhow::Result<()>
where
    T: IntoIterator<Item = &'a UnitModelEnum>,
{
    ensure!(units.into_iter().count() <= MAX_UNITS_COUNT);

    Ok(())
}
