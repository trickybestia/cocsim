use nalgebra::Vector2;
use shipyard::{
    EntityId,
    World,
};

use crate::{
    buildings::utils::passive_building::default_attack_collider,
    game::features::{
        actions::ActionEnum,
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Attacker,
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

pub fn create_active_building(
    world: &mut World,
    health: f32,
    position: Vector2<usize>,
    size: Vector2<usize>,
    min_attack_range: f32,
    max_attack_range: f32,
    attack_cooldown: f32,
    find_target: ActionEnum,
    attack: ActionEnum,
) -> EntityId {
    let collider = default_attack_collider(size);

    world.add_entity((
        Health(health),
        Building { position, size },
        CountedBuilding,
        PathfindingCollider(collider.clone()),
        Team::Defense,
        Attacker {
            min_attack_range,
            max_attack_range,
            attack_cooldown,
            target: EntityId::dead(),
            remaining_attack_cooldown: 0.0,
            find_target,
            attack,
        },
        AttackTarget {
            collider,
            flags: AttackTargetFlags::GROUND
                | AttackTargetFlags::BUILDING
                | AttackTargetFlags::COUNTED_BUILDING
                | AttackTargetFlags::ACTIVE_BUILDING,
        },
    ))
}
