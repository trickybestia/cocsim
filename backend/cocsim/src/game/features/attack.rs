use shipyard::{
    AllStoragesViewMut,
    Component,
    EntityId,
};

use crate::colliders::ColliderEnum;

#[derive(Component)]
pub struct Attacker {
    pub attack_range: f32,
    pub first_attack_cooldown: f32,
    pub attack_cooldown: f32,
    pub target: EntityId,
    pub current_attack_cooldown: f32,
    /// (self_id, all_storages) -> target_id
    pub find_target: fn(EntityId, AllStoragesViewMut) -> EntityId,
    /// (self_id, all_storages)
    pub attack: fn(EntityId, AllStoragesViewMut),
}

#[derive(Component)]
pub struct AttackTarget {
    pub collider: ColliderEnum,
    /// Like `vec!["Building", "NotWall", "ResourceBuilding", "GoldMine"]`
    pub tags: Vec<&'static str>,
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum Team {
    Attack,
    Defense,
}
