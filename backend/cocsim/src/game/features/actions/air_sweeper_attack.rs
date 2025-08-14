use std::collections::HashMap;

use hecs::Entity;

use crate::{
    Game,
    game::features::{
        actions::Action,
        attack::{
            Attacker,
            Team,
        },
        position::Position,
        projectiles::air_sweeper_projectile::AirSweeperProjectile,
    },
};

#[derive(Debug, Clone)]
pub struct AirSweeperAttack {
    pub push_strength: f32,
    pub projectile_speed: f32,
    pub start_radius: f32,
    pub max_radius: f32,
    pub angle: f32,
    pub max_arc_length: f32,
}

impl Action for AirSweeperAttack {
    fn call(&self, actor: Entity, game: &mut Game) {
        let attacker_position = game.world.get::<&Position>(actor).unwrap().0;
        let attacker_team = *game.world.get::<&Team>(actor).unwrap();
        let target = game.world.get::<&Attacker>(actor).unwrap().target;
        let target_position = game.world.get::<&Position>(target).unwrap().0;

        let target_offset = target_position - attacker_position;
        let target_angle = target_offset.y.atan2(target_offset.x).to_degrees();

        game.world.spawn((
            AirSweeperProjectile {
                push_strength: self.push_strength,
                rotation: target_angle,
                start_angle: self.angle,
                radius: self.start_radius,
                max_radius: self.max_radius,
                speed: self.projectile_speed,
                applied_push_strength: HashMap::new(),
                max_arc_length: self.max_arc_length,
            },
            Position(attacker_position),
            attacker_team,
        ));
    }
}
