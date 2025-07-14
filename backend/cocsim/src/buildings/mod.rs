mod army_camp;
mod building_data;
mod building_model;
mod building_option;
mod building_type;
pub mod colliders;
mod passive_building;

pub use army_camp::*;
pub use building_data::BuildingData;
pub use building_model::BuildingModel;
pub use building_option::BuildingOption;
pub use building_type::BuildingType;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(BuildingModel)]
pub enum BuildingModelEnum {
    ArmyCampModel,
}
