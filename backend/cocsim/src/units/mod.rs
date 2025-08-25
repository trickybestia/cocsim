mod balloon;
mod dragon;
pub mod utils;

use std::cmp::Ordering;

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
    WithHousingSpace,
    game::features::attack::Team,
};

pub struct UnitType {
    pub name: &'static str,
    pub housing_space: usize,
    pub levels: usize,
    /// See <https://clashofclans.fandom.com/wiki/Clan_Castle#Deployment_Order>.
    pub clan_castle_deployment_priority: u8,
    /// Used by ClanCastle.
    pub attack_air: bool,
    /// Used by ClanCastle.
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
    /// See <https://clashofclans.fandom.com/wiki/Clan_Castle#Deployment_Order>. "Smallest" should be deployed first
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

impl WithHousingSpace for UnitModelEnum {
    fn housing_space(&self) -> usize {
        self.r#type().housing_space
    }
}
