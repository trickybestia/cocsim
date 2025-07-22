mod melee_attack_behaviour;

use enum_dispatch::enum_dispatch;
pub use melee_attack_behaviour::MeleeAttackBehaviour;
use shipyard::{
    AllStoragesViewMut,
    EntityId,
};

#[enum_dispatch]
pub trait AttackBehaviour {
    fn attack(
        &self,
        attacker_id: EntityId,
        target_id: EntityId,
        all_storages: &mut AllStoragesViewMut,
    );
}

#[enum_dispatch(AttackBehaviour)]
#[derive(Clone)]
pub enum AttackBehaviourEnum {
    MeleeAttackBehaviour,
}
