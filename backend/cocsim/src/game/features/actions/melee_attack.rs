use hecs::Entity;

use crate::{
    Game,
    game::features::{
        actions::Action,
        attack::Attacker,
        health::EntityDamageEvent,
    },
};

#[derive(Clone, Debug)]
pub struct MeleeAttack {
    pub damage: f32,
}

impl Action for MeleeAttack {
    fn call(&self, actor: Entity, game: &mut Game) {
        let target = game.world.get::<&Attacker>(actor).unwrap().target;

        game.world.spawn((EntityDamageEvent {
            target,
            damage: self.damage,
        },));
    }
}
