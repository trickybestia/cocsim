use hecs::{
    Entity,
    Or,
};
use nalgebra::Vector2;

use crate::{
    Game,
    colliders::Collider,
    game::features::{
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Team,
        },
        position::Position,
        to_be_deleted::ToBeDeleted,
    },
};

pub struct Health(pub f32);

pub struct EntityDamageEvent {
    pub target: Entity,
    pub damage: f32,
}

pub struct SplashDamageEvent {
    pub attacker_team: Team,
    pub damage_ground: bool,
    pub damage_air: bool,
    pub target: Vector2<f32>,
    pub damage: f32,
    pub radius: f32,
}

pub fn handle_entity_damage_events(game: &mut Game) {
    for (_, damage_event) in game.world.query::<&EntityDamageEvent>().iter() {
        let mut target_health = game.world.get::<&mut Health>(damage_event.target).unwrap();

        target_health.0 -= damage_event.damage;
    }

    let to_be_deleted = game
        .world
        .query_mut::<&Health>()
        .into_iter()
        .filter(|(_, health)| health.0 <= 0.0)
        .map(|(id, _)| id)
        .collect::<Vec<_>>();

    for id in to_be_deleted {
        game.world.insert_one(id, ToBeDeleted).unwrap();
    }
}

pub fn handle_splash_damage_events(game: &mut Game) {
    let mut damage_queue = Vec::new();

    for (_, splash_damage_event) in game.world.query::<&SplashDamageEvent>().iter() {
        for (id, (position, team, attack_target)) in game
            .world
            .query::<(&Position, &Team, &AttackTarget)>()
            .iter()
        {
            if splash_damage_event.attacker_team == *team {
                continue;
            }

            if !((splash_damage_event.damage_air
                && attack_target.flags.contains(AttackTargetFlags::AIR))
                || (splash_damage_event.damage_ground
                    && attack_target.flags.contains(AttackTargetFlags::GROUND)))
            {
                continue;
            }

            if attack_target
                .collider
                .translate(position.0)
                .attack_area(splash_damage_event.radius)
                .contains(splash_damage_event.target)
            {
                damage_queue.push((EntityDamageEvent {
                    target: id,
                    damage: splash_damage_event.damage,
                },));
            }
        }
    }

    game.world.spawn_batch(damage_queue);
}

pub fn cleanup_events(game: &mut Game) {
    let despawn_queue = game
        .world
        .query_mut::<Or<&EntityDamageEvent, &SplashDamageEvent>>()
        .into_iter()
        .map(|(id, _)| id)
        .collect::<Vec<_>>();

    for id in despawn_queue {
        game.world.despawn(id).unwrap();
    }
}
