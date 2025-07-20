use nalgebra::Vector2;
use shipyard::{
    EntityId,
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

pub fn find_nearest_target(
    attacker: &Attacker,
    attacker_position: Vector2<f32>,
    targets: &[EntityId],
    v_target: View<AttackTarget>,
    v_position: View<Position>,
) -> Option<EntityId> {
    let mut result: Option<(f32, EntityId)> = None;

    for id in targets {
        let distance = v_target[*id]
            .collider
            .translate(v_position[*id].0)
            .attack_area(attacker.attack_range)
            .nearest_point(attacker_position)
            .metric_distance(&attacker_position);

        if result.is_none() || distance < result.unwrap().0 {
            result = Some((distance, *id));
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
