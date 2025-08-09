mod balloon;
mod dragon;
pub mod utils;

use std::{
    cmp::Ordering,
    fmt::Display,
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
#[serde(try_from = "Vec<UnitModelEnum>", into = "Box<[UnitModelEnum]>")]
pub struct Units<const MAX_HOUSING_SPACE: usize>(Box<[UnitModelEnum]>);

impl<const MAX_HOUSING_SPACE: usize> Units<MAX_HOUSING_SPACE> {
    pub fn new(units: &[UnitModelEnum]) -> Result<Self, HousingSpaceError> {
        let housing_space = units.iter().map(|unit| unit.r#type().housing_space).sum();

        if housing_space <= MAX_HOUSING_SPACE {
            Ok(Self(units.into()))
        } else {
            Err(HousingSpaceError {
                max: MAX_HOUSING_SPACE,
                got: housing_space,
            })
        }
    }
}

impl<const MAX_HOUSING_SPACE: usize> Deref for Units<MAX_HOUSING_SPACE> {
    type Target = [UnitModelEnum];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MAX_HOUSING_SPACE: usize> Into<Box<[UnitModelEnum]>> for Units<MAX_HOUSING_SPACE> {
    fn into(self) -> Box<[UnitModelEnum]> {
        self.0
    }
}

impl<const MAX_HOUSING_SPACE: usize> TryFrom<&[UnitModelEnum]> for Units<MAX_HOUSING_SPACE> {
    type Error = HousingSpaceError;

    fn try_from(value: &[UnitModelEnum]) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<const MAX_HOUSING_SPACE: usize> TryFrom<Vec<UnitModelEnum>> for Units<MAX_HOUSING_SPACE> {
    type Error = HousingSpaceError;

    fn try_from(value: Vec<UnitModelEnum>) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl<'a, const MAX_HOUSING_SPACE: usize> Arbitrary<'a> for Units<MAX_HOUSING_SPACE> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut result = Vec::new();
        let mut housing_space = 0;

        loop {
            let new_unit = UnitModelEnum::arbitrary(u)?;

            if housing_space + new_unit.r#type().housing_space <= MAX_HOUSING_SPACE {
                housing_space += new_unit.r#type().housing_space;
                result.push(new_unit);
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
