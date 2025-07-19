use shipyard::Component;

use crate::colliders::ColliderEnum;

#[derive(Component)]
pub struct AttackCollider(pub ColliderEnum);
