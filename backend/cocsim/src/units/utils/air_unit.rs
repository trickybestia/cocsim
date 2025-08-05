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
        actions::ActionEnum,
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Attacker,
            Team,
            UnitRetargetCondition,
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
    attack_cooldown: f32,
    attack: ActionEnum,
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
            attack_cooldown,
            remaining_attack_cooldown: attack_cooldown,
            target: EntityId::dead(),
            retarget_condition: UnitRetargetCondition.into(),
            attack,
        },
        AttackTarget {
            collider: PointCollider::zero().into(),
            flags: AttackTargetFlags::UNIT | AttackTargetFlags::AIR,
        },
        Drawable { draw_fn: draw },
    ))
}
