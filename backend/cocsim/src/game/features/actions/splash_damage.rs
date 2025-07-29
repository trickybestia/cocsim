use shipyard::{
    AllStoragesViewMut,
    EntityId,
    View,
};

use crate::game::features::{
    actions::Action,
    attack::Team,
    health::SplashDamageEvent,
    position::Position,
};

#[derive(Clone, Debug)]
pub struct SplashDamage {
    pub damage_ground: bool,
    pub damage_air: bool,
    pub damage: f32,
    pub radius: f32,
}

impl Action for SplashDamage {
    fn call(&self, actor: EntityId, all_storages: &mut AllStoragesViewMut) {
        let attacker_position = all_storages.borrow::<View<Position>>().unwrap()[actor].0;
        let attacker_team = all_storages.borrow::<View<Team>>().unwrap()[actor];

        all_storages.add_entity(SplashDamageEvent {
            attacker_team,
            damage_ground: self.damage_ground,
            damage_air: self.damage_air,
            target: attacker_position,
            damage: self.damage,
            radius: self.radius,
        });
    }
}
