use bitflags::bitflags;
use enum_dispatch::enum_dispatch;
use nalgebra::Vector2;
use shipyard::{
    AddComponent,
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
        stunned::Stunned,
        targeting::FindTargetRequest,
        time::Time,
        waypoint_mover::WaypointMover,
    },
    utils::arc_contains_angle,
};

#[enum_dispatch]
pub trait RetargetCondition {
    fn need_retarget(
        &self,
        attacker_position: Vector2<f32>,
        target_position: Vector2<f32>,
        target_collider: &ColliderEnum,
    ) -> bool;
}

pub struct BuildingRetargetCondition {
    pub min_attack_range: f32,
    pub max_attack_range: f32,
    pub rotation_angle: Option<(f32, f32)>,
}

impl RetargetCondition for BuildingRetargetCondition {
    fn need_retarget(
        &self,
        attacker_position: Vector2<f32>,
        target_position: Vector2<f32>,
        target_collider: &ColliderEnum,
    ) -> bool {
        let target_collider = target_collider.translate(target_position);

        if attacker_position.metric_distance(&target_position) < self.min_attack_range
            || !target_collider
                .attack_area(self.max_attack_range + UNIT_DISTANCE_TO_WAYPOINT_EPS)
                .contains(attacker_position)
        {
            return true;
        }

        if let Some((rotation, angle)) = self.rotation_angle {
            let target_offset = target_position - attacker_position;
            let target_angle = target_offset.y.atan2(target_offset.x).to_degrees();

            if !arc_contains_angle(rotation, angle, target_angle) {
                return true;
            }
        }

        false
    }
}

pub struct UnitRetargetCondition;

impl RetargetCondition for UnitRetargetCondition {
    fn need_retarget(
        &self,
        _attacker_position: Vector2<f32>,
        _target_position: Vector2<f32>,
        _target_collider: &ColliderEnum,
    ) -> bool {
        false
    }
}

#[enum_dispatch(RetargetCondition)]
pub enum RetargetConditionEnum {
    BuildingRetargetCondition,
    UnitRetargetCondition,
}

#[derive(Component)]
pub struct Attacker {
    pub attack_cooldown: f32,
    pub remaining_attack_cooldown: f32,
    pub target: EntityId,
    pub retarget_condition: RetargetConditionEnum,
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

pub fn create_find_target_requests(
    mut v_attacker: ViewMut<Attacker>,
    v_attack_target: View<AttackTarget>,
    v_position: View<Position>,
    entities: EntitiesView,
    v_stunned: View<Stunned>,
    mut v_find_target_request: ViewMut<FindTargetRequest>,
) {
    for (attacker_id, (attacker, stunned)) in
        (&mut v_attacker, v_stunned.as_optional()).iter().with_id()
    {
        if stunned.is_some() {
            attacker.target = EntityId::dead();

            continue;
        }

        let retarget = if !entities.is_alive(attacker.target) {
            true
        } else {
            let attacker_position = v_position[attacker_id].0;
            let target_position = v_position[attacker.target].0;
            let attack_target = &v_attack_target[attacker.target];

            attacker.retarget_condition.need_retarget(
                attacker_position,
                target_position,
                &attack_target.collider,
            )
        };

        if retarget {
            attacker.target = EntityId::dead();
            attacker.remaining_attack_cooldown = attacker.attack_cooldown;

            v_find_target_request.add_component_unchecked(attacker_id, FindTargetRequest);
        }
    }
}

pub fn attack(mut all_storages: AllStoragesViewMut) {
    let attack_queue = all_storages.run(create_attack_queue);

    for (attack, attacker_id) in attack_queue {
        attack.call(attacker_id, &mut all_storages);
    }
}

fn create_attack_queue(
    time: UniqueView<Time>,
    mut v_attacker: ViewMut<Attacker>,
    entities: EntitiesView,
    v_waypoint_mover: View<WaypointMover>,
) -> Vec<(ActionEnum, EntityId)> {
    let mut result = Vec::new();

    for (attacker_id, attacker) in (&mut v_attacker).iter().with_id() {
        if entities.is_alive(attacker.target) {
            let can_attack = if let Ok(waypoint_mover) = v_waypoint_mover.get(attacker_id) {
                // attacker is unit

                waypoint_mover.waypoints.is_empty()
            } else {
                // attacker is building

                true
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
