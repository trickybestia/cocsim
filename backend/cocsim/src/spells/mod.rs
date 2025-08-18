mod haste_spell;
mod lightning_spell;
mod utils;

use arbitrary::Arbitrary;
use enum_dispatch::enum_dispatch;
pub use haste_spell::*;
pub use lightning_spell::*;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    Game,
    WithHousingSpace,
};

#[derive(Serialize)]
pub struct SpellType {
    pub name: &'static str,
    pub housing_space: usize,
    pub levels: usize,
}

inventory::collect!(SpellType);

#[enum_dispatch]
pub trait SpellModel {
    fn r#type(&self) -> &'static SpellType;

    fn level(&self) -> usize;

    fn spawn(&self, game: &mut Game, position: Vector2<f32>);
}

#[enum_dispatch(SpellModel)]
#[derive(Serialize, Deserialize, Debug, Clone, Arbitrary)]
#[serde(tag = "name")]
pub enum SpellModelEnum {
    #[serde(rename = "Haste")]
    HasteSpellModel,
    #[serde(rename = "Lightning")]
    LightningSpellModel,
}

impl WithHousingSpace for SpellModelEnum {
    fn housing_space(&self) -> usize {
        self.r#type().housing_space
    }
}
