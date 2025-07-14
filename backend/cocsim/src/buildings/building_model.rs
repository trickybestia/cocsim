use enum_dispatch::enum_dispatch;

use crate::Building;

#[enum_dispatch]
pub trait BuildingModel {
    fn create_building(&self) -> Box<dyn Building>;
}
