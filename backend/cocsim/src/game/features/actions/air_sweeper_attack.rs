use std::collections::HashMap;

use shipyard::{
    AllStoragesViewMut,
    EntityId,
    View,
};

use crate::game::features::{
    actions::Action,
    attack::{
        Attacker,
        Team,
    },
    position::Position,
    projectiles::air_sweeper_projectile::AirSweeperProjectile,
};

#[derive(Clone, Debug)]
pub struct AirSweeperAttack {
    pub push_strength: f32,
    pub projectile_speed: f32,
    pub start_radius: f32,
    pub max_radius: f32,
    pub angle: f32,
    pub max_arc_length: f32,
}

impl Action for AirSweeperAttack {
    fn call(&self, actor: EntityId, all_storages: &mut AllStoragesViewMut) {
        let target = all_storages.borrow::<View<Attacker>>().unwrap()[actor].target;
        let v_position = all_storages.borrow::<View<Position>>().unwrap();
        let v_team = all_storages.borrow::<View<Team>>().unwrap();

        let attacker_position = v_position[actor].0;
        let target_position = v_position[target].0;
        let attacker_team = v_team[actor];

        drop(v_position);
        drop(v_team);

        let target_offset = target_position - attacker_position;
        let target_angle = target_offset.y.atan2(target_offset.x).to_degrees();

        all_storages.add_entity((
            AirSweeperProjectile {
                push_strength: self.push_strength,
                rotation: target_angle,
                start_angle: self.angle,
                radius: self.start_radius,
                max_radius: self.max_radius,
                speed: self.projectile_speed,
                applied_push_strength: HashMap::new(),
                max_arc_length: self.max_arc_length,
            },
            Position(attacker_position),
            attacker_team,
        ));
    }
}
