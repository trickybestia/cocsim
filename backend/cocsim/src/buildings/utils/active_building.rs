use nalgebra::Vector2;
use shipyard::{
    EntityId,
    World,
};

use crate::{
    buildings::utils::passive_building::{
        default_attack_collider,
        default_pathfinding_collider,
    },
    game::features::{
        actions::ActionEnum,
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Attacker,
            RetargetConditionEnum,
            Team,
        },
        buildings::{
            Building,
            CountedBuilding,
        },
        health::Health,
    },
};

pub fn create_active_building(
    world: &mut World,
    health: f32,
    position: Vector2<usize>,
    size: Vector2<usize>,
    attack_cooldown: f32,
    retarget_condition: RetargetConditionEnum,
    attack: ActionEnum,
) -> EntityId {
    world.add_entity((
        Health(health),
        Building { position, size },
        CountedBuilding,
        default_pathfinding_collider(size),
        Team::Defense,
        Attacker {
            attack_cooldown,
            remaining_attack_cooldown: attack_cooldown,
            target: EntityId::dead(),
            retarget_condition,
            attack,
        },
        AttackTarget {
            collider: default_attack_collider(size),
            flags: AttackTargetFlags::GROUND
                | AttackTargetFlags::BUILDING
                | AttackTargetFlags::COUNTED_BUILDING
                | AttackTargetFlags::ACTIVE_BUILDING,
        },
    ))
}
