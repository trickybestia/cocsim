use enum_dispatch::enum_dispatch;
use hecs::{
    Entity,
    PreparedQuery,
};

use crate::{
    Game,
    colliders::Collider,
    game::features::{
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Attacker,
            Team,
        },
        position::Position,
        waypoint_mover::WaypointMover,
    },
    utils::AnyMapExt,
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

pub struct AirUnitFindTarget {
    pub prioritizer: TargetPrioritizerEnum,
    pub attack_range: f32,
}

#[derive(Default)]
struct HandleRetargetCache<'a> {
    pub attacker_query: PreparedQuery<(
        &'a AirUnitFindTarget,
        &'a mut Attacker,
        &'a Team,
        &'a Position,
        &'a mut WaypointMover,
    )>,
    pub target_query: PreparedQuery<(&'a AttackTarget, &'a Team, &'a Position)>,
}

pub fn handle_retarget(game: &mut Game) {
    struct NearestTarget {
        pub id: Entity,
        pub flags: AttackTargetFlags,
        pub distance: f32,
    }

    let cache = game.cache.get_mut_or_default::<HandleRetargetCache>();

    for (
        _attacker_id,
        (air_unit_find_target, attacker, attacker_team, attacker_position, attacker_waypoint_mover),
    ) in cache.attacker_query.query(&game.world).iter()
    {
        if !attacker.retarget {
            continue;
        }

        attacker.retarget = false;

        let mut nearest_target: Option<NearestTarget> = None;

        for (target_id, (attack_target, target_team, target_position)) in
            cache.target_query.query(&game.world).iter()
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
                .translate(target_position.0)
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
            attacker_waypoint_mover.waypoints = vec![
                game.world
                    .get::<&AttackTarget>(nearest_target.id)
                    .unwrap()
                    .collider
                    .translate(game.world.get::<&Position>(nearest_target.id).unwrap().0)
                    .attack_area(air_unit_find_target.attack_range)
                    .random_near_point(attacker_position.0, &mut game.rng),
            ];
            attacker.target = nearest_target.id;
        }
    }
}
