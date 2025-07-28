use nalgebra::Vector2;
use shipyard::{
    AllStoragesView,
    EntityId,
    World,
};

use crate::{
    Shape,
    colliders::PointCollider,
    game::features::{
        attack::{
            AttackBehaviourEnum,
            AttackTarget,
            AttackTargetFlags,
            Attacker,
            FindTargetBehaviourEnum,
            Team,
        },
        drawable::Drawable,
        health::Health,
        position::Position,
        waypoint_mover::WaypointMover,
    },
};

pub fn create_air_unit(
    world: &mut World,
    position: Vector2<f32>,
    health: f32,
    speed: f32,
    max_attack_range: f32,
    attack_cooldown: f32,
    find_target_behaviour: FindTargetBehaviourEnum,
    attack_behaviour: AttackBehaviourEnum,
    draw: fn(EntityId, &AllStoragesView, &mut Vec<Shape>),
) -> EntityId {
    world.add_entity((
        Position(position),
        WaypointMover {
            speed,
            waypoints: Vec::new(),
        },
        Health(health),
        Team::Attack,
        Attacker {
            min_attack_range: 0.0,
            max_attack_range,
            attack_cooldown,
            target: EntityId::dead(),
            remaining_attack_cooldown: 0.0,
            find_target_behaviour,
            attack_behaviour,
        },
        AttackTarget {
            collider: PointCollider::zero().into(),
            flags: AttackTargetFlags::UNIT | AttackTargetFlags::AIR,
        },
        Drawable { draw_fn: draw },
    ))
}
