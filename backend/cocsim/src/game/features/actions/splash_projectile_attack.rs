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
    projectiles::splash_projectile::SplashProjectile,
};

#[derive(Clone, Debug)]
pub struct SplashProjectileAttack {
    pub damage: f32,
    pub damage_radius: f32,
    pub damage_air: bool,
    pub damage_ground: bool,
    pub projectile_speed: f32,
}

impl Action for SplashProjectileAttack {
    fn call(&self, actor: EntityId, all_storages: &mut AllStoragesViewMut) {
        let target = all_storages.borrow::<View<Attacker>>().unwrap()[actor].target;
        let v_position = all_storages.borrow::<View<Position>>().unwrap();
        let v_team = all_storages.borrow::<View<Team>>().unwrap();

        let attacker_position = v_position[actor].0;
        let target_position = v_position[target].0;
        let attacker_team = v_team[actor];

        drop(v_position);
        drop(v_team);

        all_storages.add_entity((
            SplashProjectile {
                damage: self.damage,
                damage_radius: self.damage_radius,
                damage_air: self.damage_air,
                damage_ground: self.damage_ground,
                target: target_position,
                speed: self.projectile_speed,
            },
            Position(attacker_position),
            attacker_team,
        ));
    }
}
