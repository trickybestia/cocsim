mod air_sweeper_attack;
mod delayed;
mod empty_action;
mod melee_attack;
mod splash_damage;
mod splash_projectile_attack;
mod target_projectile_attack;

pub use air_sweeper_attack::AirSweeperAttack;
pub use delayed::Delayed;
pub use empty_action::EmptyAction;
use enum_dispatch::enum_dispatch;
use hecs::Entity;
pub use melee_attack::MeleeAttack;
pub use splash_damage::SplashDamage;
pub use splash_projectile_attack::SplashProjectileAttack;
pub use target_projectile_attack::TargetProjectileAttack;

use crate::Game;

#[enum_dispatch]
pub trait Action {
    fn call(&self, actor: Entity, game: &mut Game);
}

#[enum_dispatch(Action)]
#[derive(Clone, Debug)]
pub enum ActionEnum {
    AirSweeperAttack,
    Delayed,
    EmptyAction,
    MeleeAttack,
    SplashDamage,
    SplashProjectileAttack,
    TargetProjectileAttack,
}
