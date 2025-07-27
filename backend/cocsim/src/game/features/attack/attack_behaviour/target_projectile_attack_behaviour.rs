use shipyard::{
    AllStoragesViewMut,
    EntityId,
    View,
};

use crate::game::features::{
    attack::AttackBehaviour,
    position::Position,
    projectiles::target_projectile::TargetProjectile,
};

#[derive(Clone)]
pub struct TargetProjectileAttackBehaviour {
    pub damage: f32,
    pub projectile_speed: f32,
}

impl AttackBehaviour for TargetProjectileAttackBehaviour {
    fn attack(
        &self,
        attacker_id: EntityId,
        target_id: EntityId,
        all_storages: &mut AllStoragesViewMut,
    ) {
        let v_position = all_storages.borrow::<View<Position>>().unwrap();

        let attacker_position = v_position[attacker_id].0;
        let target_position = v_position[target_id].0;

        drop(v_position);

        all_storages.add_entity((
            TargetProjectile {
                damage: self.damage,
                target: target_id,
                relative_position: attacker_position - target_position,
                speed: self.projectile_speed,
            },
            Position(attacker_position),
        ));
    }
}
