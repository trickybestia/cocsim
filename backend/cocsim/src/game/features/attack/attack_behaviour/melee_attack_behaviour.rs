use shipyard::{
    AllStoragesViewMut,
    EntityId,
};

use crate::game::features::{
    attack::AttackBehaviour,
    health::DamageEvent,
};

#[derive(Clone)]
pub struct MeleeAttackBehaviour {
    pub damage: f32,
}

impl AttackBehaviour for MeleeAttackBehaviour {
    fn attack(
        &self,
        attacker_id: EntityId,
        target_id: EntityId,
        all_storages: &mut AllStoragesViewMut,
    ) {
        all_storages.add_entity((DamageEvent {
            target: target_id,
            damage: self.damage,
        },));
    }
}
