use nalgebra::Vector2;
use shipyard::{
    EntityId,
    World,
};

use crate::{
    colliders::{
        ColliderEnum,
        RectCollider,
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
        collision::PathfindingCollider,
        health::Health,
    },
};

pub fn default_attack_collider(size: Vector2<usize>) -> ColliderEnum {
    RectCollider::new_from_center(Vector2::new(0.0, 0.0), size.cast() * 0.65).into()
}

pub fn create_passive_building(
    world: &mut World,
    health: f32,
    position: Vector2<usize>,
    size: Vector2<usize>,
    attack_collider: Option<ColliderEnum>,
) -> EntityId {
    let collider = match attack_collider {
        Some(collider) => collider,
        None => default_attack_collider(size),
    };

    world.add_entity((
        Health(health),
        Building { position, size },
        CountedBuilding,
        PathfindingCollider(collider.clone()),
        Team::Defense,
        AttackTarget {
            collider,
            flags: AttackTargetFlags::GROUND
                | AttackTargetFlags::BUILDING
                | AttackTargetFlags::COUNTED_BUILDING,
        },
    ))
}
