use nalgebra::Vector2;
use shipyard::{
    EntityId,
    Get,
    IntoIter,
    View,
};

use crate::{
    colliders::Collider,
    game::features::{
        attack::{
            AttackTarget,
            Attacker,
        },
        position::Position,
    },
};

pub fn find_targets(attacker: &Attacker, v_target: View<AttackTarget>) -> Vec<EntityId> {
    let mut result = Vec::new();

    for priority in &attacker.priorities {
        for (id, target) in v_target.iter().with_id() {
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

pub fn find_nearest_target(
    attacker: &Attacker,
    attacker_position: Vector2<f32>,
    v_target: View<AttackTarget>,
    v_position: View<Position>,
) -> Option<EntityId> {
    let targets = find_targets(attacker, v_target.clone());

    let mut result: Option<(f32, EntityId)> = None;

    for id in targets {
        let distance = v_target[id]
            .collider
            .translate(v_position[id].0)
            .attack_area(attacker.attack_range)
            .nearest_point(attacker_position)
            .metric_distance(&attacker_position);

        if result.is_none() || distance < result.unwrap().0 {
            result = Some((distance, id));
        }
    }

    result.map(|(_, id)| id)
}

pub fn find_air_path_end(
    attacker: &Attacker,
    attacker_position: Vector2<f32>,
    target: &AttackTarget,
    target_position: Vector2<f32>,
) -> Vector2<f32> {
    target
        .collider
        .translate(target_position)
        .attack_area(attacker.attack_range)
        .nearest_point(attacker_position)
}

pub fn find
