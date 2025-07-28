use shipyard::{
    AllStoragesViewMut,
    EntityId,
    View,
};

use crate::game::features::{
    attack::{
        AttackBehaviour,
        Team,
    },
    position::Position,
    projectiles::splash_projectile::SplashProjectile,
};

#[derive(Clone)]
pub struct SplashProjectileAttackBehaviour {
    pub damage: f32,
    pub damage_radius: f32,
    pub damage_air: bool,
    pub damage_ground: bool,
    pub projectile_speed: f32,
}

impl AttackBehaviour for SplashProjectileAttackBehaviour {
    fn attack(
        &self,
        attacker_id: EntityId,
        target_id: EntityId,
        all_storages: &mut AllStoragesViewMut,
    ) {
        let v_position = all_storages.borrow::<View<Position>>().unwrap();
        let v_team = all_storages.borrow::<View<Team>>().unwrap();

        let attacker_position = v_position[attacker_id].0;
        let target_position = v_position[target_id].0;
        let attacker_team = v_team[attacker_id];

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
