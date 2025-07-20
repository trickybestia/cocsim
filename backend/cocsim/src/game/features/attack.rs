use shipyard::Component;

use crate::colliders::ColliderEnum;

#[derive(Component)]
pub struct Attacker {
    pub attack_range: f32,
    /// Like vec!["AirDefense", "DefenseBuilding", "NotWall"]
    pub priorities: Vec<String>,
    pub team: Team,
}

#[derive(Component)]
pub struct AttackTarget {
    pub collider: ColliderEnum,
    /// Like `vec!["Building", "NotWall", "ResourceBuilding", "GoldMine"]`
    pub tags: Vec<String>,
    pub team: Team,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Team {
    Attack,
    Defense,
}
