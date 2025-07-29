use shipyard::{
    AllStoragesViewMut,
    EntityId,
    IntoIter,
    View,
    ViewMut,
};

use crate::{
    colliders::Collider,
    consts::UNIT_DISTANCE_TO_WAYPOINT_EPS,
    game::features::{
        actions::Action,
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Attacker,
            Team,
        },
        position::Position,
    },
};

#[derive(Clone, Debug)]
pub struct BuildingFindTarget {
    pub attack_air: bool,
    pub attack_ground: bool,
}

impl Action for BuildingFindTarget {
    fn call(&self, actor: EntityId, all_storages: &mut AllStoragesViewMut) {
        let mut v_attacker = all_storages.borrow::<ViewMut<Attacker>>().unwrap();
        let v_attack_target = all_storages.borrow::<View<AttackTarget>>().unwrap();
        let v_team = all_storages.borrow::<View<Team>>().unwrap();
        let v_position = all_storages.borrow::<View<Position>>().unwrap();

        let attacker = &mut v_attacker[actor];
        let attacker_team = v_team[actor];
        let attacker_position = v_position[actor].0;

        let mut nearest_target = EntityId::dead();
        let mut nearest_target_distance: f32 = f32::INFINITY;

        for (target_id, (attack_target, team, position)) in
            (&v_attack_target, &v_team, &v_position).iter().with_id()
        {
            if *team == attacker_team {
                continue;
            }

            if !((self.attack_air && attack_target.flags.contains(AttackTargetFlags::AIR))
                || (self.attack_ground && attack_target.flags.contains(AttackTargetFlags::GROUND)))
            {
                continue;
            }

            let attack_target_collider = attack_target.collider.translate(position.0);
            let min_attack_range_collider =
                attack_target_collider.attack_area(attacker.min_attack_range);
            let max_attack_range_collider = attack_target_collider
                .attack_area(attacker.max_attack_range + UNIT_DISTANCE_TO_WAYPOINT_EPS);

            if min_attack_range_collider.contains(attacker_position)
                || !max_attack_range_collider.contains(attacker_position)
            {
                continue;
            }

            let distance = max_attack_range_collider
                .nearest_point(attacker_position)
                .metric_distance(&attacker_position);

            if distance < nearest_target_distance {
                nearest_target = target_id;
                nearest_target_distance = distance;
            }
        }

        attacker.target = nearest_target;
    }
}
