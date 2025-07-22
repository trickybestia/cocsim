mod air_unit_find_target_behaviour;
mod building_find_target_behaviour;

pub use air_unit_find_target_behaviour::AirUnitFindTargetBehaviour;
pub use building_find_target_behaviour::BuildingFindTargetBehaviour;
use enum_dispatch::enum_dispatch;
use shipyard::{
    AllStoragesViewMut,
    EntityId,
};

#[enum_dispatch]
pub trait FindTargetBehaviour {
    fn find_target(&self, attacker_id: EntityId, all_storages: &AllStoragesViewMut);
}

#[enum_dispatch(FindTargetBehaviour)]
#[derive(Clone)]
pub enum FindTargetBehaviourEnum {
    AirUnitFindTargetBehaviour,
    BuildingFindTargetBehaviour,
}
