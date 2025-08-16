use hecs::Entity;

use crate::{
    Game,
    game::features::{
        actions::Action,
        attack::Attacker,
        health::Health,
    },
};

#[derive(Debug, Clone)]
pub struct MeleeAttack {
    pub damage: f32,
}

impl Action for MeleeAttack {
    fn call(&self, actor: Entity, game: &mut Game) {
        let target = game.world.get::<&Attacker>(actor).unwrap().target;
        let mut target_health = game.world.get::<&mut Health>(target).unwrap();

        target_health.incoming_damage += self.damage;
    }
}
