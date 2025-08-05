use hecs::{
    Entity,
    PreparedQuery,
};

use crate::{
    Game,
    colliders::Collider,
    consts::UNIT_DISTANCE_TO_WAYPOINT_EPS,
    game::features::{
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Attacker,
            Team,
        },
        position::Position,
    },
    utils::{
        AnyMapExt,
        arc_contains_angle,
    },
};

pub struct BuildingFindTarget {
    pub attack_air: bool,
    pub attack_ground: bool,
    pub rotation_angle: Option<(f32, f32)>,
    pub min_attack_range: f32,
    pub max_attack_range: f32,
}

#[derive(Default)]
struct HandleRetargetCache<'a> {
    pub attacker_query: PreparedQuery<(
        &'a BuildingFindTarget,
        &'a mut Attacker,
        &'a Team,
        &'a Position,
    )>,
    pub target_query: PreparedQuery<(&'a AttackTarget, &'a Team, &'a Position)>,
}

pub fn handle_retarget(game: &mut Game) {
    let cache = game.cache.get_mut_or_default::<HandleRetargetCache>();

    for (_attacker_id, (building_find_target, attacker, attacker_team, attacker_position)) in
        cache.attacker_query.query(&game.world).iter()
    {
        if !attacker.retarget {
            continue;
        }

        attacker.retarget = false;

        let mut nearest_target = Entity::DANGLING;
        let mut nearest_target_distance_squared = f32::INFINITY;

        for (target_id, (attack_target, target_team, target_position)) in
            cache.target_query.query(&game.world).iter()
        {
            if target_team == attacker_team {
                continue;
            }

            if !((building_find_target.attack_air
                && attack_target.flags.contains(AttackTargetFlags::AIR))
                || (building_find_target.attack_ground
                    && attack_target.flags.contains(AttackTargetFlags::GROUND)))
            {
                continue;
            }

            if let Some((rotation, angle)) = building_find_target.rotation_angle {
                let target_offset = target_position.0 - attacker_position.0;
                let target_angle = target_offset.y.atan2(target_offset.x).to_degrees();

                if !arc_contains_angle(rotation, angle, target_angle) {
                    continue;
                }
            }

            let attack_target_collider = attack_target.collider.translate(target_position.0);
            let min_attack_range_collider =
                attack_target_collider.attack_area(building_find_target.min_attack_range);
            let max_attack_range_collider = attack_target_collider
                .attack_area(building_find_target.max_attack_range + UNIT_DISTANCE_TO_WAYPOINT_EPS);

            if min_attack_range_collider.contains(attacker_position.0)
                || !max_attack_range_collider.contains(attacker_position.0)
            {
                continue;
            }

            let distance_squared = (max_attack_range_collider.nearest_point(attacker_position.0)
                - attacker_position.0)
                .norm_squared();

            if distance_squared < nearest_target_distance_squared {
                nearest_target = target_id;
                nearest_target_distance_squared = distance_squared;
            }
        }

        attacker.target = nearest_target;
    }
}
