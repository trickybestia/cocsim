use enum_dispatch::enum_dispatch;
use shipyard::{
    Component,
    EntityId,
    IntoIter,
    UniqueViewMut,
    View,
    ViewMut,
};

use crate::{
    colliders::Collider,
    game::features::{
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Attacker,
            Team,
        },
        position::Position,
        rng::RngUnique,
        targeting::FindTargetRequest,
        waypoint_mover::WaypointMover,
    },
};

#[enum_dispatch]
pub trait TargetPrioritizer {
    fn can_attack(&self, flags: AttackTargetFlags) -> bool;

    fn is_better(
        &self,
        current_flags: AttackTargetFlags,
        current_distance: f32,
        other_flags: AttackTargetFlags,
        other_distance: f32,
    ) -> bool;
}

#[derive(Clone, Debug)]
pub struct CountedBuildingTargetPrioritizer;

impl TargetPrioritizer for CountedBuildingTargetPrioritizer {
    fn can_attack(&self, flags: AttackTargetFlags) -> bool {
        flags.contains(AttackTargetFlags::COUNTED_BUILDING)
    }

    fn is_better(
        &self,
        _current_flags: AttackTargetFlags,
        current_distance: f32,
        _other_flags: AttackTargetFlags,
        other_distance: f32,
    ) -> bool {
        other_distance < current_distance
    }
}

#[derive(Clone, Debug)]
pub struct ActiveBuildingTargetPrioritizer;

impl TargetPrioritizer for ActiveBuildingTargetPrioritizer {
    fn can_attack(&self, flags: AttackTargetFlags) -> bool {
        flags.contains(AttackTargetFlags::COUNTED_BUILDING)
    }

    fn is_better(
        &self,
        current_flags: AttackTargetFlags,
        current_distance: f32,
        other_flags: AttackTargetFlags,
        other_distance: f32,
    ) -> bool {
        if current_flags.contains(AttackTargetFlags::ACTIVE_BUILDING) {
            other_flags.contains(AttackTargetFlags::ACTIVE_BUILDING)
                && other_distance < current_distance
        } else {
            other_flags.contains(AttackTargetFlags::ACTIVE_BUILDING)
                || other_distance < current_distance
        }
    }
}

#[enum_dispatch(TargetPrioritizer)]
#[derive(Clone, Debug)]
pub enum TargetPrioritizerEnum {
    ActiveBuildingTargetPrioritizer,
    CountedBuildingTargetPrioritizer,
}

#[derive(Component)]
pub struct AirUnitFindTarget {
    pub prioritizer: TargetPrioritizerEnum,
    pub attack_range: f32,
}

pub fn handle_find_target_requests(
    mut rng: UniqueViewMut<RngUnique>,
    v_find_target_request: ViewMut<FindTargetRequest>,
    v_air_unit_find_target: View<AirUnitFindTarget>,
    mut v_attacker: ViewMut<Attacker>,
    v_attack_target: View<AttackTarget>,
    v_team: View<Team>,
    v_position: View<Position>,
    mut v_waypoint_mover: ViewMut<WaypointMover>,
) {
    struct NearestTarget {
        pub id: EntityId,
        pub flags: AttackTargetFlags,
        pub distance: f32,
    }

    for (attacker_id, (_, air_unit_find_target, attacker, attacker_team, attacker_position)) in (
        &v_find_target_request,
        &v_air_unit_find_target,
        &mut v_attacker,
        &v_team,
        &v_position,
    )
        .iter()
        .with_id()
    {
        let mut nearest_target: Option<NearestTarget> = None;

        for (target_id, (attack_target, target_team)) in
            (&v_attack_target, &v_team).iter().with_id()
        {
            if target_team == attacker_team
                || !air_unit_find_target
                    .prioritizer
                    .can_attack(attack_target.flags)
            {
                continue;
            }

            let nearest_point = attack_target
                .collider
                .translate(v_position[target_id].0)
                .attack_area(air_unit_find_target.attack_range)
                .nearest_point(attacker_position.0);
            let distance = nearest_point.metric_distance(&attacker_position.0);

            let mut update_target = false;

            if let Some(nearest_target) = &mut nearest_target {
                if air_unit_find_target.prioritizer.is_better(
                    nearest_target.flags,
                    nearest_target.distance,
                    attack_target.flags,
                    distance,
                ) {
                    update_target = true;
                }
            } else {
                update_target = true;
            }

            if update_target {
                nearest_target = Some(NearestTarget {
                    id: target_id,
                    flags: attack_target.flags,
                    distance,
                });
            }
        }

        if let Some(nearest_target) = nearest_target {
            v_waypoint_mover[attacker_id].waypoints = vec![
                v_attack_target[nearest_target.id]
                    .collider
                    .translate(v_position[nearest_target.id].0)
                    .attack_area(air_unit_find_target.attack_range)
                    .random_near_point(attacker_position.0, &mut rng.0),
            ];
            attacker.target = nearest_target.id;
        }
    }
}
