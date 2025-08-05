use hecs::Entity;

use crate::{
    Game,
    game::features::{
        actions::Action,
        attack::Attacker,
        position::Position,
        projectiles::target_projectile::TargetProjectile,
    },
};

#[derive(Clone, Debug)]
pub struct TargetProjectileAttack {
    pub damage: f32,
    pub projectile_speed: f32,
}

impl Action for TargetProjectileAttack {
    fn call(&self, actor: Entity, game: &mut Game) {
        let attacker_position = game.world.get::<&Position>(actor).unwrap().0;
        let target = game.world.get::<&Attacker>(actor).unwrap().target;
        let target_position = game.world.get::<&Position>(target).unwrap().0;

        let relative_position = attacker_position - target_position;

        game.world.spawn((
            TargetProjectile {
                damage: self.damage,
                target,
                relative_position,
                speed: self.projectile_speed,
                remaining_time: relative_position.norm() / self.projectile_speed,
            },
            Position(attacker_position),
        ));
    }
}
