use hecs::Entity;

use crate::{
    Game,
    game::features::{
        actions::Action,
        attack::Team,
        health::SplashDamageEvent,
        position::Position,
    },
};

#[derive(Debug, Clone)]
pub struct SplashDamage {
    pub damage_ground: bool,
    pub damage_air: bool,
    pub damage: f32,
    pub radius: f32,
}

impl Action for SplashDamage {
    fn call(&self, actor: Entity, game: &mut Game) {
        let attacker_position = game.world.get::<&Position>(actor).unwrap().0;
        let attacker_team = *game.world.get::<&Team>(actor).unwrap();

        game.world.spawn((SplashDamageEvent {
            attacker_team,
            damage_ground: self.damage_ground,
            damage_air: self.damage_air,
            target: attacker_position,
            damage: self.damage,
            radius: self.radius,
        },));
    }
}
