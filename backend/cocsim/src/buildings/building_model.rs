use enum_dispatch::enum_dispatch;
use shipyard::World;

#[enum_dispatch]
pub trait BuildingModel {
    fn create_building(&self, world: &mut World);
}
