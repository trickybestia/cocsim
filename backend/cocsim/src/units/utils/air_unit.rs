use hecs::{
    Entity,
    World,
};
use nalgebra::Vector2;

use crate::{
    colliders::PointCollider,
    game::features::{
        actions::Action,
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Attacker,
            FalseRetargetCondition,
            Team,
        },
        drawable::Drawable,
        health::Health,
        mover::Mover,
        position::Position,
        unit::Unit,
    },
};

pub fn spawn_air_unit(
    world: &mut World,
    position: Vector2<f32>,
    health: f32,
    speed: f32,
    attack_cooldown: f32,
    attack: Box<dyn Action>,
    drawable: Drawable,
    team: Team,
    housing_space: usize,
) -> Entity {
    world.spawn((
        Position(position),
        Unit { housing_space },
        Mover {
            speed,
            arrived: true,
            target: position,
        },
        Health {
            health,
            incoming_damage: 0.0,
        },
        team,
        Attacker {
            attack_cooldown,
            remaining_attack_cooldown: attack_cooldown,
            target: Entity::DANGLING,
            retarget_condition: FalseRetargetCondition.into(),
            retarget: true,
            attack,
        },
        AttackTarget {
            collider: PointCollider::zero().into(),
            flags: AttackTargetFlags::UNIT | AttackTargetFlags::AIR,
        },
        drawable,
    ))
}
