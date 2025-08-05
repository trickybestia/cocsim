use hecs::{
    Entity,
    Or,
    PreparedQuery,
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
    utils::AnyMapExt,
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
    for (_, damage_event) in game
        .cache
        .get_mut_or_default::<PreparedQuery<&EntityDamageEvent>>()
        .query(&game.world)
        .iter()
    {
        let mut target_health = game.world.get::<&mut Health>(damage_event.target).unwrap();

        target_health.0 -= damage_event.damage;
    }

    let to_be_deleted = game
        .cache
        .get_mut_or_default::<PreparedQuery<&Health>>()
        .query_mut(&mut game.world)
        .into_iter()
        .filter(|(_, health)| health.0 <= 0.0)
        .map(|(id, _)| id)
        .collect::<Vec<_>>();

    for id in to_be_deleted {
        game.world.insert_one(id, ToBeDeleted).unwrap();
    }
}

#[derive(Default)]
struct HandleSplashDamageEventsCache<'a> {
    pub event_query: PreparedQuery<&'a SplashDamageEvent>,
    pub target_query: PreparedQuery<(&'a AttackTarget, &'a Position, &'a Team)>,
}

pub fn handle_splash_damage_events(game: &mut Game) {
    let cache = game
        .cache
        .get_mut_or_default::<HandleSplashDamageEventsCache>();

    let mut damage_queue = Vec::new();

    for (_, splash_damage_event) in cache.event_query.query(&game.world).iter() {
        for (id, (attack_target, position, team)) in cache.target_query.query(&game.world).iter() {
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
        .cache
        .get_mut_or_default::<PreparedQuery<Or<&EntityDamageEvent, &SplashDamageEvent>>>()
        .query_mut(&mut game.world)
        .into_iter()
        .map(|(id, _)| id)
        .collect::<Vec<_>>();

    for id in despawn_queue {
        game.world.despawn(id).unwrap();
    }
}
