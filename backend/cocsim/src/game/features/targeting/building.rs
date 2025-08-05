use shipyard::{
    Component,
    EntityId,
    IntoIter,
    View,
    ViewMut,
};

use crate::{
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
        targeting::FindTargetRequest,
    },
    utils::arc_contains_angle,
};

#[derive(Component)]
pub struct BuildingFindTarget {
    pub attack_air: bool,
    pub attack_ground: bool,
    pub rotation_angle: Option<(f32, f32)>,
    pub min_attack_range: f32,
    pub max_attack_range: f32,
}

pub fn handle_find_target_requests(
    v_find_target_request: ViewMut<FindTargetRequest>,
    v_building_find_target: View<BuildingFindTarget>,
    mut v_attacker: ViewMut<Attacker>,
    v_attack_target: View<AttackTarget>,
    v_team: View<Team>,
    v_position: View<Position>,
) {
    for (_, building_find_target, attacker, attacker_team, attacker_position) in (
        &v_find_target_request,
        &v_building_find_target,
        &mut v_attacker,
        &v_team,
        &v_position,
    )
        .iter()
    {
        let mut nearest_target = EntityId::dead();
        let mut nearest_target_distance_squared = f32::INFINITY;

        for (target_id, (attack_target, target_team, target_position)) in
            (&v_attack_target, &v_team, &v_position).iter().with_id()
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
