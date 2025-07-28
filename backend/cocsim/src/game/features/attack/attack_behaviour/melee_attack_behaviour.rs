use shipyard::{
    AllStoragesViewMut,
    EntityId,
};

use crate::game::features::{
    attack::AttackBehaviour,
    health::EntityDamageEvent,
};

#[derive(Clone)]
pub struct MeleeAttackBehaviour {
    pub damage: f32,
}

impl AttackBehaviour for MeleeAttackBehaviour {
    fn attack(
        &self,
        _attacker_id: EntityId,
        target_id: EntityId,
        all_storages: &mut AllStoragesViewMut,
    ) {
        all_storages.add_entity((EntityDamageEvent {
            target: target_id,
            damage: self.damage,
        },));
    }
}
