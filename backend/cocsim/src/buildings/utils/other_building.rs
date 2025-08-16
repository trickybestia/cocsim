use hecs::{
    Entity,
    World,
};
use nalgebra::Vector2;

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
        buildings::Building,
        collision::PathfindingCollider,
        health::Health,
        position::Position,
    },
};

pub fn default_pathfinding_collider(size: Vector2<usize>) -> PathfindingCollider {
    assert!(size.x == size.y);

    match size.x {
        1 => PathfindingCollider {
            position: Vector2::new(0, 0),
            size: Vector2::new(2, 2),
        },
        2 => PathfindingCollider {
            position: Vector2::new(1, 1),
            size: Vector2::new(2, 2),
        },
        3 => PathfindingCollider {
            position: Vector2::new(1, 1),
            size: Vector2::new(4, 4),
        },
        4 => PathfindingCollider {
            position: Vector2::new(2, 2),
            size: Vector2::new(4, 4),
        },
        _ => panic!("Invalid size {}", size),
    }
}

pub fn default_attack_collider(size: Vector2<usize>) -> ColliderEnum {
    assert!(size.x == size.y);

    match size.x {
        1 => RectCollider::new_from_center(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0)).into(),
        2 => RectCollider::new_from_center(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0)).into(),
        3 => RectCollider::new_from_center(Vector2::new(0.0, 0.0), Vector2::new(2.0, 2.0)).into(),
        4 => RectCollider::new_from_center(Vector2::new(0.0, 0.0), Vector2::new(3.0, 3.0)).into(),
        _ => panic!("Invalid size {}", size),
    }
}

pub fn spawn_other_building(
    world: &mut World,
    health: f32,
    position: Vector2<usize>,
    size: Vector2<usize>,
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
        AttackTarget {
            collider: default_attack_collider(size),
            flags: AttackTargetFlags::GROUND | AttackTargetFlags::OTHER_BUILDING,
        },
    ))
}
