mod air_unit_find_target;
mod building_find_target;
mod delayed;
mod empty_action;
mod melee_attack;
mod splash_damage;
mod splash_projectile_attack;
mod target_projectile_attack;

pub use air_unit_find_target::AirUnitFindTarget;
pub use building_find_target::BuildingFindTarget;
pub use delayed::Delayed;
pub use empty_action::EmptyAction;
use enum_dispatch::enum_dispatch;
pub use melee_attack::MeleeAttack;
use shipyard::{
    AllStoragesViewMut,
    EntityId,
};
pub use splash_damage::SplashDamage;
pub use splash_projectile_attack::SplashProjectileAttack;
pub use target_projectile_attack::TargetProjectileAttack;

#[enum_dispatch]
pub trait Action {
    fn call(&self, actor: EntityId, all_storages: &mut AllStoragesViewMut);
}

#[enum_dispatch(Action)]
#[derive(Clone, Debug)]
pub enum ActionEnum {
    AirUnitFindTarget,
    BuildingFindTarget,
    Delayed,
    EmptyAction,
    MeleeAttack,
    SplashDamage,
    SplashProjectileAttack,
    TargetProjectileAttack,
}
