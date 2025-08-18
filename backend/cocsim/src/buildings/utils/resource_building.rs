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
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Team,
        },
        buildings::Building,
        health::Health,
        position::Position,
    },
};

pub fn spawn_resource_building(
    world: &mut World,
    health: f32,
    position: Vector2<usize>,
    size: Vector2<usize>,
) -> Entity {
    world.spawn((
        Health {
            health,
            max_health: health,
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
        AttackTarget {
            collider: default_attack_collider(size),
            flags: AttackTargetFlags::GROUND | AttackTargetFlags::RESOURCE_BUILDING,
        },
    ))
}
