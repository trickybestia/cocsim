use shipyard::{
    AllStoragesViewMut,
    EntityId,
    View,
};

use crate::game::features::{
    actions::Action,
    attack::Attacker,
    health::EntityDamageEvent,
};

#[derive(Clone, Debug)]
pub struct MeleeAttack {
    pub damage: f32,
}

impl Action for MeleeAttack {
    fn call(&self, actor: EntityId, all_storages: &mut AllStoragesViewMut) {
        let target = all_storages.borrow::<View<Attacker>>().unwrap()[actor].target;

        all_storages.add_entity((EntityDamageEvent {
            target,
            damage: self.damage,
        },));
    }
}
