mod lightning_spell;
mod utils;

use std::{
    iter::repeat,
    ops::Deref,
};

use arbitrary::Arbitrary;
use enum_dispatch::enum_dispatch;
pub use lightning_spell::*;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    Game,
    HousingSpaceError,
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
    #[serde(rename = "LightningSpell")]
    LightningSpellModel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpellWithCount {
    pub value: SpellModelEnum,
    pub count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "Vec<SpellWithCount>", into = "Box<[SpellWithCount]>")]
pub struct SpellsWithCount<const MAX_HOUSING_SPACE: usize>(Box<[SpellWithCount]>);

impl<const MAX_HOUSING_SPACE: usize> SpellsWithCount<MAX_HOUSING_SPACE> {
    pub fn new(spells: &[SpellWithCount]) -> Result<Self, HousingSpaceError> {
        let housing_space = spells
            .iter()
            .map(|spell_with_count| {
                spell_with_count.value.r#type().housing_space * spell_with_count.count
            })
            .sum();

        if housing_space <= MAX_HOUSING_SPACE {
            Ok(Self(spells.into()))
        } else {
            Err(HousingSpaceError {
                max: MAX_HOUSING_SPACE,
                got: housing_space,
            })
        }
    }

    pub fn flatten(&self) -> impl Iterator<Item = SpellModelEnum> {
        self.0
            .iter()
            .map(|spell_with_count| {
                repeat(spell_with_count.value.clone()).take(spell_with_count.count)
            })
            .flatten()
    }
}

impl<const MAX_HOUSING_SPACE: usize> Deref for SpellsWithCount<MAX_HOUSING_SPACE> {
    type Target = [SpellWithCount];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MAX_HOUSING_SPACE: usize> Into<Box<[SpellWithCount]>>
    for SpellsWithCount<MAX_HOUSING_SPACE>
{
    fn into(self) -> Box<[SpellWithCount]> {
        self.0
    }
}

impl<const MAX_HOUSING_SPACE: usize> TryFrom<&[SpellWithCount]>
    for SpellsWithCount<MAX_HOUSING_SPACE>
{
    type Error = HousingSpaceError;

    fn try_from(value: &[SpellWithCount]) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<const MAX_HOUSING_SPACE: usize> TryFrom<Vec<SpellWithCount>>
    for SpellsWithCount<MAX_HOUSING_SPACE>
{
    type Error = HousingSpaceError;

    fn try_from(value: Vec<SpellWithCount>) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl<'a, const MAX_HOUSING_SPACE: usize> Arbitrary<'a> for SpellsWithCount<MAX_HOUSING_SPACE> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut result = Vec::new();
        let mut housing_space = 0;

        loop {
            let spell = SpellModelEnum::arbitrary(u)?;

            if housing_space + spell.r#type().housing_space <= MAX_HOUSING_SPACE {
                housing_space += spell.r#type().housing_space;
                result.push(SpellWithCount {
                    value: spell,
                    count: 1,
                });
            } else {
                return Ok(Self(result.into_boxed_slice()));
            }
        }
    }
}
