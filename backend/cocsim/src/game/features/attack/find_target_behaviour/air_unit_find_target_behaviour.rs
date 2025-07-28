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
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Attacker,
            FindTargetBehaviour,
            Team,
        },
        position::Position,
        waypoint_mover::WaypointMover,
    },
};

#[derive(Clone)]
pub struct AirUnitFindTargetBehaviour {
    pub pattern: AttackTargetFlags,
}

impl FindTargetBehaviour for AirUnitFindTargetBehaviour {
    fn find_target(&self, attacker_id: EntityId, all_storages: &AllStoragesViewMut) {
        let mut v_attacker = all_storages.borrow::<ViewMut<Attacker>>().unwrap();
        let v_attack_target = all_storages.borrow::<View<AttackTarget>>().unwrap();
        let v_team = all_storages.borrow::<View<Team>>().unwrap();
        let v_position = all_storages.borrow::<View<Position>>().unwrap();

        let attacker = &mut v_attacker[attacker_id];
        let attacker_team = v_team[attacker_id];
        let attacker_position = v_position[attacker_id].0;

        let mut nearest_target = EntityId::dead();
        let mut nearest_target_nearest_point = Vector2::zeros();
        let mut nearest_target_distance: f32 = f32::INFINITY;

        for (target_id, (attack_target, team)) in (&v_attack_target, &v_team).iter().with_id() {
            assert!(attacker.min_attack_range == 0.0);

            if *team == attacker_team || !attack_target.flags.contains(self.pattern) {
                continue;
            }

            let nearest_point = attack_target
                .collider
                .translate(v_position[target_id].0)
                .attack_area(attacker.max_attack_range)
                .nearest_point(attacker_position);
            let distance = nearest_point.metric_distance(&attacker_position);

            if distance < nearest_target_distance {
                nearest_target = target_id;
                nearest_target_nearest_point = nearest_point;
                nearest_target_distance = distance;
            }
        }

        if nearest_target != EntityId::dead() {
            let mut v_waypoint_mover = all_storages.borrow::<ViewMut<WaypointMover>>().unwrap();

            v_waypoint_mover[attacker_id].waypoints = vec![nearest_target_nearest_point];
        }

        attacker.target = nearest_target;
    }
}
