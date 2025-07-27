use nalgebra::Vector2;
use shipyard::{
    EntityId,
    World,
};

use crate::{
    buildings::utils::passive_building::default_attack_collider,
    game::features::{
        attack::{
            AttackBehaviourEnum,
            AttackTarget,
            AttackTargetFlags,
            Attacker,
            FindTargetBehaviourEnum,
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
    attack_range: f32,
    attack_cooldown: f32,
    find_target_behaviour: FindTargetBehaviourEnum,
    attack_behaviour: AttackBehaviourEnum,
) -> EntityId {
    let collider = default_attack_collider(size);

    world.add_entity((
        Health(health),
        Building { position, size },
        CountedBuilding,
        PathfindingCollider(collider.clone()),
        Team::Defense,
        Attacker {
            attack_range,
            attack_cooldown,
            target: EntityId::dead(),
            remaining_attack_cooldown: 0.0,
            find_target_behaviour,
            attack_behaviour,
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
