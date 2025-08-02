use bitflags::bitflags;
use shipyard::{
    AllStoragesViewMut,
    Component,
    EntitiesView,
    EntityId,
    Get,
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
        actions::{
            Action,
            ActionEnum,
        },
        position::Position,
        time::Time,
        trapped::Trapped,
        waypoint_mover::WaypointMover,
    },
};

#[derive(Component)]
pub struct Attacker {
    pub min_attack_range: f32,
    pub max_attack_range: f32,
    pub attack_cooldown: f32,
    pub target: EntityId,
    pub remaining_attack_cooldown: f32,
    pub find_target: ActionEnum,
    pub attack: ActionEnum,
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

        const COUNTED_BUILDING = 1 << 4;

        const ACTIVE_BUILDING = 1 << 5;
    }
}

#[derive(Component, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Team {
    Attack,
    Defense,
}

pub fn find_target(mut all_storages: AllStoragesViewMut) {
    let find_target_queue = all_storages.run(create_find_target_queue);

    for (find_target, id) in find_target_queue {
        find_target.call(id, &mut all_storages);
    }
}

pub fn attack(mut all_storages: AllStoragesViewMut) {
    let attack_queue = all_storages.run(create_attack_queue);

    for (attack, attacker_id) in attack_queue {
        attack.call(attacker_id, &mut all_storages);
    }
}

fn create_find_target_queue(
    mut v_attacker: ViewMut<Attacker>,
    v_attack_target: View<AttackTarget>,
    v_position: View<Position>,
    entities: EntitiesView,
    v_waypoint_mover: View<WaypointMover>,
    v_trapped: View<Trapped>,
) -> Vec<(ActionEnum, EntityId)> {
    let mut result = Vec::new();

    for (attacker_id, (attacker, trapped)) in
        (&mut v_attacker, v_trapped.as_optional()).iter().with_id()
    {
        if trapped.is_some() {
            attacker.target = EntityId::dead();

            continue;
        }

        let retarget = if !entities.is_alive(attacker.target) {
            true
        } else if v_waypoint_mover.get(attacker_id).is_ok() {
            // attacker is unit, it's moving to target, just wait

            false
        } else {
            // attacker is building

            let attacker_position = v_position[attacker_id].0;
            let target_position = v_position[attacker.target].0;
            let attack_target = &v_attack_target[attacker.target];
            let attack_target_collider = attack_target.collider.translate(target_position);

            attack_target_collider
                .attack_area(attacker.min_attack_range)
                .contains(attacker_position)
                || !attack_target_collider
                    .attack_area(attacker.max_attack_range + UNIT_DISTANCE_TO_WAYPOINT_EPS)
                    .contains(attacker_position)
        };

        if retarget {
            result.push((attacker.find_target.clone(), attacker_id));

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
    v_waypoint_mover: View<WaypointMover>,
) -> Vec<(ActionEnum, EntityId)> {
    let mut result = Vec::new();

    for (attacker_id, attacker) in (&mut v_attacker).iter().with_id() {
        if entities.is_alive(attacker.target) {
            let attacker_position = v_position[attacker_id].0;
            let attack_target = &v_attack_target[attacker.target];
            let attack_target_position = v_position[attacker.target].0;
            let attack_target_collider = attack_target.collider.translate(attack_target_position);

            let can_attack = if let Ok(waypoint_mover) = v_waypoint_mover.get(attacker_id) {
                // attacker is unit
                waypoint_mover.waypoints.is_empty()
            } else {
                // attacker is building
                attacker_position.metric_distance(&attack_target_position)
                    >= attacker.min_attack_range
                    && attack_target_collider
                        .attack_area(attacker.max_attack_range + UNIT_DISTANCE_TO_WAYPOINT_EPS)
                        .contains(attacker_position) // target is in attack range
            };

            if can_attack {
                attacker.remaining_attack_cooldown =
                    0.0f32.max(attacker.remaining_attack_cooldown - time.delta);

                if attacker.remaining_attack_cooldown == 0.0 {
                    result.push((attacker.attack.clone(), attacker_id));

                    attacker.remaining_attack_cooldown = attacker.attack_cooldown;
                }
            }
        }
    }

    result
}
