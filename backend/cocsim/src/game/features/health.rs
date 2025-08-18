use hecs::PreparedQuery;
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
        to_be_despawned::ToBeDespawned,
    },
    utils::AnyMapExt,
};

pub struct Health {
    pub health: f32,
    pub max_health: f32,
    /// Can be both positive (damage) and negative (heal).
    pub incoming_damage: f32,
}

pub struct SplashDamageEvent {
    pub attacker_team: Team,
    pub damage_ground: bool,
    pub damage_air: bool,
    pub target: Vector2<f32>,
    pub damage: f32,
    pub radius: f32,
}

pub fn handle_incoming_damage(game: &mut Game) {
    let mut to_be_despawned = Vec::new();

    for (id, health) in game
        .cache
        .get_mut_or_default::<PreparedQuery<&mut Health>>()
        .query_mut(&mut game.world)
    {
        health.health = health
            .max_health
            .min(health.health - health.incoming_damage);
        health.incoming_damage = 0.0;

        if health.health <= 0.0 {
            to_be_despawned.push(id);
        }
    }

    for id in to_be_despawned {
        game.world.insert_one(id, ToBeDespawned).unwrap();
    }
}

#[derive(Default)]
struct HandleSplashDamageEventsCache<'a> {
    pub event_query: PreparedQuery<&'a SplashDamageEvent>,
    pub target_query: PreparedQuery<(&'a AttackTarget, &'a mut Health, &'a Position, &'a Team)>,
}

pub fn handle_splash_damage_events(game: &mut Game) {
    let cache = game
        .cache
        .get_mut_or_default::<HandleSplashDamageEventsCache>();

    let mut to_be_despawned = Vec::new();

    for (event_id, splash_damage_event) in cache.event_query.query(&game.world).iter() {
        for (_, (attack_target, health, position, team)) in
            cache.target_query.query(&game.world).iter()
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
                health.incoming_damage += splash_damage_event.damage;
            }
        }

        to_be_despawned.push(event_id);
    }

    for id in to_be_despawned {
        game.world.despawn(id).unwrap();
    }
}
