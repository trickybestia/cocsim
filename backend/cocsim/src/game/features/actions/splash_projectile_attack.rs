use hecs::Entity;

use crate::{
    Game,
    game::features::{
        actions::Action,
        attack::{
            Attacker,
            Team,
        },
        damage::DamageMultiplier,
        position::Position,
        projectiles::splash_projectile::SplashProjectile,
    },
};

#[derive(Debug, Clone)]
pub struct SplashProjectileAttack {
    pub damage: f32,
    pub damage_radius: f32,
    pub damage_air: bool,
    pub damage_ground: bool,
    pub projectile_speed: f32,
}

impl Action for SplashProjectileAttack {
    fn call(&self, actor: Entity, game: &mut Game) {
        let attacker_position = game.world.get::<&Position>(actor).unwrap().0;
        let attacker_team = *game.world.get::<&Team>(actor).unwrap();
        let target = game.world.get::<&Attacker>(actor).unwrap().target;
        let target_position = game.world.get::<&Position>(target).unwrap().0;

        let damage_multiplier = game
            .world
            .get::<&DamageMultiplier>(actor)
            .map(|m| m.value)
            .unwrap_or(1.0);

        game.world.spawn((
            SplashProjectile {
                damage: self.damage * damage_multiplier,
                damage_radius: self.damage_radius,
                damage_air: self.damage_air,
                damage_ground: self.damage_ground,
                target: target_position,
                speed: self.projectile_speed,
                remaining_time: (attacker_position - target_position).norm()
                    / self.projectile_speed,
            },
            Position(attacker_position),
            attacker_team,
        ));
    }
}
