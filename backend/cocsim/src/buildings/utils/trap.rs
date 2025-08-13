use hecs::{
    Entity,
    World,
};
use nalgebra::Vector2;

use crate::game::features::{
    actions::ActionEnum,
    attack::{
        Attacker,
        FalseRetargetCondition,
        Team,
    },
    buildings::Building,
    position::Position,
};

pub fn spawn_trap(
    world: &mut World,
    position: Vector2<usize>,
    size: Vector2<usize>,
    attack: ActionEnum,
) -> Entity {
    world.spawn((
        Position(position.cast() + size.cast() / 2.0),
        Building {
            position,
            size,
            affects_drop_zone: false,
            affects_percentage_destroyed: false,
        },
        Attacker {
            attack_cooldown: 0.0,
            remaining_attack_cooldown: 0.0,
            target: Entity::DANGLING,
            retarget_condition: FalseRetargetCondition.into(),
            retarget: true,
            attack,
        },
        Team::Defense,
    ))
}
