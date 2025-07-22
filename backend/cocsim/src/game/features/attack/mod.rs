mod attack_behaviour;
mod find_target_behaviour;

pub use attack_behaviour::*;
use bitflags::bitflags;
pub use find_target_behaviour::*;
use shipyard::{
    AllStoragesViewMut,
    Component,
    EntitiesView,
    EntityId,
    IntoIter,
    UniqueView,
    View,
    ViewMut,
};

use crate::{
    colliders::{
        Collider,
        ColliderEnum,
    },
    consts::*,
    game::features::{
        position::Position,
        time::Time,
    },
};

#[derive(Component)]
pub struct Attacker {
    pub attack_range: f32,
    pub attack_cooldown: f32,
    pub target: EntityId,
    pub remaining_attack_cooldown: f32,
    pub find_target_behaviour: FindTargetBehaviourEnum,
    pub attack_behaviour: AttackBehaviourEnum,
}

#[derive(Component)]
pub struct AttackTarget {
    pub collider: ColliderEnum,
    pub flags: AttackTargetFlags,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AttackTargetFlags: u8 {
        const BUILDING = 1;
        const UNIT = 1 << 1;

        const AIR = 1 << 2;
        const GROUND = 1 << 3;

        const PASSIVE_BUILDING = 1 << 4;
        const WALL = 1 << 5;
    }
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum Team {
    Attack,
    Defense,
}

pub fn find_target(all_storages: AllStoragesViewMut) {
    let find_target_queue = all_storages.run(create_find_target_queue);

    for (find_target_behaviour, id) in find_target_queue {
        find_target_behaviour.find_target(id, &all_storages);
    }
}

pub fn attack(mut all_storages: AllStoragesViewMut) {
    let attack_queue = all_storages.run(create_attack_queue);

    for (attack_behaviour, attacker_id, target_id) in attack_queue {
        attack_behaviour.attack(attacker_id, target_id, &mut all_storages);
    }
}

fn create_find_target_queue(
    mut v_attacker: ViewMut<Attacker>,
    entities: EntitiesView,
) -> Vec<(FindTargetBehaviourEnum, EntityId)> {
    let mut result = Vec::new();

    for (id, attacker) in (&mut v_attacker).iter().with_id() {
        if !entities.is_alive(attacker.target) {
            result.push((attacker.find_target_behaviour.clone(), id));

            attacker.remaining_attack_cooldown = attacker.attack_cooldown;
        }
    }

    result
}

fn create_attack_queue(
    time: UniqueView<Time>,
    mut v_attacker: ViewMut<Attacker>,
    entities: EntitiesView,
    v_attack_target: View<AttackTarget>,
    v_position: View<Position>,
) -> Vec<(AttackBehaviourEnum, EntityId, EntityId)> {
    let mut result = Vec::new();

    for (attacker_id, attacker) in (&mut v_attacker).iter().with_id() {
        if entities.is_alive(attacker.target) {
            let attacker_position = v_position[attacker_id].0;
            let attack_target = &v_attack_target[attacker.target];
            let attack_target_position = v_position[attacker.target].0;

            let attack_area = attack_target
                .collider
                .translate(attack_target_position)
                .attack_area(attacker.attack_range + DISTANCE_TO_WAYPOINT_EPS);

            if attack_area.contains(attacker_position) {
                attacker.remaining_attack_cooldown =
                    0.0f32.max(attacker.remaining_attack_cooldown - time.delta);

                if attacker.remaining_attack_cooldown == 0.0 {
                    result.push((
                        attacker.attack_behaviour.clone(),
                        attacker_id,
                        attacker.target,
                    ));

                    attacker.remaining_attack_cooldown = attacker.attack_cooldown;
                }
            }
        }
    }

    result
}
