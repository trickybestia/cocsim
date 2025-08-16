use hecs::{
    Entity,
    World,
};
use nalgebra::Vector2;

use crate::{
    buildings::utils::other_building::{
        default_attack_collider,
        default_pathfinding_collider,
    },
    game::features::{
        actions::Action,
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Attacker,
            RetargetConditionEnum,
            Team,
        },
        buildings::Building,
        health::Health,
        position::Position,
    },
};

pub fn spawn_defensive_building(
    world: &mut World,
    health: f32,
    position: Vector2<usize>,
    size: Vector2<usize>,
    attack_cooldown: f32,
    retarget_condition: RetargetConditionEnum,
    attack: Box<dyn Action>,
) -> Entity {
    world.spawn((
        Health {
            health,
            incoming_damage: 0.0,
        },
        Position(position.cast() + size.cast() / 2.0),
        Building {
            position,
            size,
            affects_drop_zone: true,
            affects_percentage_destroyed: true,
        },
        default_pathfinding_collider(size),
        Team::Defense,
        Attacker {
            attack_cooldown,
            remaining_attack_cooldown: attack_cooldown,
            target: Entity::DANGLING,
            retarget_condition,
            retarget: true,
            attack,
        },
        AttackTarget {
            collider: default_attack_collider(size),
            flags: AttackTargetFlags::GROUND | AttackTargetFlags::DEFENSIVE_BUILDING,
        },
    ))
}
