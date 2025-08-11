mod balloon;
mod dragon;
pub mod utils;

use std::{
    cmp::Ordering,
    fmt::Display,
    iter::repeat,
    ops::Deref,
};

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

use crate::game::features::attack::Team;

#[derive(Serialize)]
pub struct UnitType {
    pub name: &'static str,
    pub housing_space: usize,
    pub levels: usize,
    /// https://clashofclans.fandom.com/wiki/Clan_Castle#Deployment_Order
    #[serde(skip)]
    pub clan_castle_deployment_priority: u8,
    /// Used by ClanCastle.
    #[serde(skip)]
    pub attack_air: bool,
    /// Used by ClanCastle.
    #[serde(skip)]
    pub attack_ground: bool,
}

inventory::collect!(UnitType);

#[enum_dispatch]
pub trait UnitModel {
    fn r#type(&self) -> &'static UnitType;

    fn level(&self) -> usize;

    fn spawn(&self, world: &mut World, position: Vector2<f32>, team: Team);
}

#[enum_dispatch(UnitModel)]
#[derive(Serialize, Deserialize, Debug, Clone, Arbitrary)]
#[serde(tag = "name")]
pub enum UnitModelEnum {
    #[serde(rename = "Balloon")]
    BalloonModel,
    #[serde(rename = "Dragon")]
    DragonModel,
}

impl UnitModelEnum {
    /// "Smallest" should be deployed first
    pub fn clan_castle_deployment_cmp(&self, other: &Self) -> Ordering {
        let housing_space_ord = self
            .r#type()
            .housing_space
            .cmp(&other.r#type().housing_space);

        if housing_space_ord != Ordering::Equal {
            housing_space_ord
        } else {
            let clan_castle_deployment_priority_ord = self
                .r#type()
                .clan_castle_deployment_priority
                .cmp(&other.r#type().clan_castle_deployment_priority);

            if clan_castle_deployment_priority_ord != Ordering::Equal {
                clan_castle_deployment_priority_ord
            } else {
                self.level().cmp(&other.level())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnitWithCount {
    pub unit: UnitModelEnum,
    pub count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "Vec<UnitWithCount>", into = "Box<[UnitWithCount]>")]
pub struct UnitsWithCount<const MAX_HOUSING_SPACE: usize>(Box<[UnitWithCount]>);

impl<const MAX_HOUSING_SPACE: usize> UnitsWithCount<MAX_HOUSING_SPACE> {
    pub fn new(units: &[UnitWithCount]) -> Result<Self, HousingSpaceError> {
        let housing_space = units
            .iter()
            .map(|unit_with_count| {
                unit_with_count.unit.r#type().housing_space * unit_with_count.count
            })
            .sum();

        if housing_space <= MAX_HOUSING_SPACE {
            Ok(Self(units.into()))
        } else {
            Err(HousingSpaceError {
                max: MAX_HOUSING_SPACE,
                got: housing_space,
            })
        }
    }

    pub fn flatten(&self) -> impl Iterator<Item = UnitModelEnum> {
        self.0
            .iter()
            .map(|unit_with_count| repeat(unit_with_count.unit.clone()).take(unit_with_count.count))
            .flatten()
    }
}

impl<const MAX_HOUSING_SPACE: usize> Deref for UnitsWithCount<MAX_HOUSING_SPACE> {
    type Target = [UnitWithCount];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MAX_HOUSING_SPACE: usize> Into<Box<[UnitWithCount]>>
    for UnitsWithCount<MAX_HOUSING_SPACE>
{
    fn into(self) -> Box<[UnitWithCount]> {
        self.0
    }
}

impl<const MAX_HOUSING_SPACE: usize> TryFrom<&[UnitWithCount]>
    for UnitsWithCount<MAX_HOUSING_SPACE>
{
    type Error = HousingSpaceError;

    fn try_from(value: &[UnitWithCount]) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<const MAX_HOUSING_SPACE: usize> TryFrom<Vec<UnitWithCount>>
    for UnitsWithCount<MAX_HOUSING_SPACE>
{
    type Error = HousingSpaceError;

    fn try_from(value: Vec<UnitWithCount>) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl<'a, const MAX_HOUSING_SPACE: usize> Arbitrary<'a> for UnitsWithCount<MAX_HOUSING_SPACE> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut result = Vec::new();
        let mut housing_space = 0;

        loop {
            let unit = UnitModelEnum::arbitrary(u)?;

            if housing_space + unit.r#type().housing_space <= MAX_HOUSING_SPACE {
                housing_space += unit.r#type().housing_space;
                result.push(UnitWithCount { unit, count: 1 });
            } else {
                return Ok(Self(result.into_boxed_slice()));
            }
        }
    }
}

#[derive(Debug)]
pub struct HousingSpaceError {
    pub max: usize,
    pub got: usize,
}

impl Display for HousingSpaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Housing space is larger than MAX_HOUSING_SPACE (MAX_HOUSING_SPACE = {}, got = {})",
            self.max, self.got
        )?;

        Ok(())
    }
}
