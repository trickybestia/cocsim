use enum_dispatch::enum_dispatch;

use crate::BuildingData;

#[enum_dispatch]
pub trait BuildingModel {
    fn create_building(&self) -> Box<dyn BuildingData>;
}
