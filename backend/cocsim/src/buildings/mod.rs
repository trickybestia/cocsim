use std::rc::Rc;

mod army_camp;
mod building;
mod building_model;
mod building_option;
mod building_type;
pub mod colliders;

pub use army_camp::*;
pub use building::Building;
pub use building_model::BuildingModel;
pub use building_option::BuildingOption;
pub use building_type::BuildingType;
use enum_dispatch::enum_dispatch;
use serde::Deserialize;

#[enum_dispatch(BuildingModel)]
#[derive(Deserialize)]
pub enum BuildingModelEnum {
    ArmyCampModel,
}
