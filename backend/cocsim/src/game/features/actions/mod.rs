mod air_sweeper_attack;
mod melee_attack;
mod splash_damage;
mod splash_projectile_attack;
mod target_projectile_attack;
mod with_delay;
mod with_despawn;

use std::fmt::Debug;

pub use air_sweeper_attack::AirSweeperAttack;
use dyn_clone::DynClone;
use hecs::Entity;
pub use melee_attack::MeleeAttack;
pub use splash_damage::SplashDamage;
pub use splash_projectile_attack::SplashProjectileAttack;
pub use target_projectile_attack::TargetProjectileAttack;
pub use with_delay::WithDelay;
pub use with_despawn::WithDespawn;

use crate::Game;

pub trait Action: Debug + Send + Sync + DynClone {
    fn call(&self, actor: Entity, game: &mut Game);
}

dyn_clone::clone_trait_object!(Action);
