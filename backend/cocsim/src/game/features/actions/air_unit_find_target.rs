use enum_dispatch::enum_dispatch;
use nalgebra::Vector2;
use shipyard::{
    AllStoragesViewMut,
    EntityId,
    IntoIter,
    View,
    ViewMut,
};

use crate::{
    colliders::Collider,
    game::features::{
        actions::Action,
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Attacker,
            Team,
        },
        position::Position,
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

#[derive(Clone, Debug)]
pub struct AirUnitFindTarget {
    pub prioritizer: TargetPrioritizerEnum,
}

impl Action for AirUnitFindTarget {
    fn call(&self, actor: EntityId, all_storages: &mut AllStoragesViewMut) {
        let mut v_attacker = all_storages.borrow::<ViewMut<Attacker>>().unwrap();
        let v_attack_target = all_storages.borrow::<View<AttackTarget>>().unwrap();
        let v_team = all_storages.borrow::<View<Team>>().unwrap();
        let v_position = all_storages.borrow::<View<Position>>().unwrap();

        let attacker = &mut v_attacker[actor];
        let attacker_team = v_team[actor];
        let attacker_position = v_position[actor].0;

        struct NearestTarget {
            pub id: EntityId,
            pub flags: AttackTargetFlags,
            pub nearest_point: Vector2<f32>,
            pub distance: f32,
        }

        let mut nearest_target: Option<NearestTarget> = None;

        for (target_id, (attack_target, team)) in (&v_attack_target, &v_team).iter().with_id() {
            assert!(attacker.min_attack_range == 0.0);

            if *team == attacker_team || !self.prioritizer.can_attack(attack_target.flags) {
                continue;
            }

            let nearest_point = attack_target
                .collider
                .translate(v_position[target_id].0)
                .attack_area(attacker.max_attack_range)
                .nearest_point(attacker_position);
            let distance = nearest_point.metric_distance(&attacker_position);

            let mut update_target = false;

            if let Some(nearest_target) = &mut nearest_target {
                if self.prioritizer.is_better(
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
                    nearest_point,
                    distance,
                });
            }
        }

        if let Some(nearest_target) = nearest_target {
            let mut v_waypoint_mover = all_storages.borrow::<ViewMut<WaypointMover>>().unwrap();

            v_waypoint_mover[actor].waypoints = vec![nearest_target.nearest_point];
            attacker.target = nearest_target.id;
        }
    }
}
