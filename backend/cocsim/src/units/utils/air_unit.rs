use hecs::{
    Entity,
    World,
};
use nalgebra::Vector2;

use crate::{
    Game,
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
        mover::Mover,
        position::Position,
    },
};

pub fn create_air_unit(
    world: &mut World,
    position: Vector2<f32>,
    health: f32,
    speed: f32,
    attack_cooldown: f32,
    attack: ActionEnum,
    draw: fn(Entity, &Game, &mut Vec<Shape>),
    team: Team,
) -> Entity {
    world.spawn((
        Position(position),
        Mover {
            speed,
            arrived: true,
            target: position,
        },
        Health(health),
        team,
        Attacker {
            attack_cooldown,
            remaining_attack_cooldown: attack_cooldown,
            target: Entity::DANGLING,
            retarget_condition: UnitRetargetCondition.into(),
            retarget: true,
            attack,
        },
        AttackTarget {
            collider: PointCollider::zero().into(),
            flags: AttackTargetFlags::UNIT | AttackTargetFlags::AIR,
        },
        Drawable { draw_fn: draw },
    ))
}
