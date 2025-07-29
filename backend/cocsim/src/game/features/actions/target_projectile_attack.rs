use shipyard::{
    AllStoragesViewMut,
    EntityId,
    View,
};

use crate::game::features::{
    actions::Action,
    attack::Attacker,
    position::Position,
    projectiles::target_projectile::TargetProjectile,
};

#[derive(Clone, Debug)]
pub struct TargetProjectileAttack {
    pub damage: f32,
    pub projectile_speed: f32,
}

impl Action for TargetProjectileAttack {
    fn call(&self, actor: EntityId, all_storages: &mut AllStoragesViewMut) {
        let target = all_storages.borrow::<View<Attacker>>().unwrap()[actor].target;
        let v_position = all_storages.borrow::<View<Position>>().unwrap();

        let attacker_position = v_position[actor].0;
        let target_position = v_position[target].0;

        drop(v_position);

        all_storages.add_entity((
            TargetProjectile {
                damage: self.damage,
                target,
                relative_position: attacker_position - target_position,
                speed: self.projectile_speed,
            },
            Position(attacker_position),
        ));
    }
}
