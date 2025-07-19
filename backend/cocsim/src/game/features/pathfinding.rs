use shipyard::{
    EntityId,
    IntoIter,
    View,
};

use crate::game::features::attack::{
    AttackTarget,
    Attacker,
};

pub fn find_targets(attacker: &Attacker, targets: View<AttackTarget>) -> Vec<EntityId> {
    let mut result = Vec::new();

    for priority in &attacker.priorities {
        for (id, target) in targets.iter().with_id() {
            if target.team != attacker.team && target.tags.contains(priority) {
                result.push(id);
            }
        }

        if !result.is_empty() {
            break;
        }
    }

    result
}
