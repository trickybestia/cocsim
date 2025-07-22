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
    },
};

#[derive(Clone)]
pub struct BuildingFindTargetBehaviour {
    pub attack_air: bool,
    pub attack_ground: bool,
}

impl FindTargetBehaviour for BuildingFindTargetBehaviour {
    fn find_target(&self, attacker_id: EntityId, all_storages: &AllStoragesViewMut) {
        let mut v_attacker = all_storages.borrow::<ViewMut<Attacker>>().unwrap();
        let v_attack_target = all_storages.borrow::<View<AttackTarget>>().unwrap();
        let v_team = all_storages.borrow::<View<Team>>().unwrap();
        let v_position = all_storages.borrow::<View<Position>>().unwrap();

        let attacker = &mut v_attacker[attacker_id];
        let attacker_team = v_team[attacker_id];
        let attacker_position = v_position[attacker_id].0;

        let mut nearest_target = EntityId::dead();
        let mut nearest_target_distance: f32 = f32::INFINITY;

        for (target_id, (attack_target, team)) in (&v_attack_target, &v_team).iter().with_id() {
            if *team == attacker_team {
                continue;
            }

            if !((self.attack_air && attack_target.flags.contains(AttackTargetFlags::AIR))
                || (self.attack_ground && attack_target.flags.contains(AttackTargetFlags::GROUND)))
            {
                continue;
            }

            let distance = attack_target
                .collider
                .translate(v_position[target_id].0)
                .attack_area(attacker.attack_range)
                .nearest_point(attacker_position)
                .metric_distance(&attacker_position);

            if distance < attacker.attack_range && distance < nearest_target_distance {
                nearest_target = target_id;
                nearest_target_distance = distance;
            }
        }

        attacker.target = nearest_target;
    }
}
