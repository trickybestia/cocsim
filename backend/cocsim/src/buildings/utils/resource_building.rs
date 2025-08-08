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
        buildings::{
            Building,
            CountedBuilding,
        },
        health::Health,
    },
};

pub fn spawn_resource_building(
    world: &mut World,
    health: f32,
    position: Vector2<usize>,
    size: Vector2<usize>,
) -> Entity {
    world.spawn((
        Health(health),
        Building { position, size },
        CountedBuilding,
        default_pathfinding_collider(size),
        Team::Defense,
        AttackTarget {
            collider: default_attack_collider(size),
            flags: AttackTargetFlags::GROUND | AttackTargetFlags::RESOURCE_BUILDING,
        },
    ))
}
