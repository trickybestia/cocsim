use shipyard::{
    AllStoragesViewMut,
    EntityId,
};

use crate::game::features::actions::Action;

#[derive(Clone, Debug)]
pub struct EmptyAction;

impl Action for EmptyAction {
    fn call(&self, _actor: EntityId, _all_storages: &mut AllStoragesViewMut) {}
}
