mod army_camp;
mod building_model;
mod building_option;
mod building_type;

pub use army_camp::*;
pub use building_model::BuildingModel;
pub use building_option::BuildingOption;
pub use building_type::BuildingType;
use enum_dispatch::enum_dispatch;
use serde::{
    Deserialize,
    Serialize,
};
use shipyard::World;

#[enum_dispatch(BuildingModel)]
#[derive(Serialize, Deserialize, Debug)]
pub enum BuildingModelEnum {
    ArmyCampModel,
}
