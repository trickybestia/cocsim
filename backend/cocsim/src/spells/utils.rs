use std::f32::consts::SQRT_2;

use hecs::{
    Entity,
    World,
};
use nalgebra::Vector2;

use crate::game::features::{
    actions::Action,
    delay::Delay,
    drawable::Drawable,
    mover::Mover,
    position::Position,
    to_be_despawned::OnDespawn,
};

const SPELL_DROP_DELAY: f32 = 0.8;
const SPELL_DROP_OFFSET: f32 = 5.0;
const SPELL_DROP_SPEED: f32 = SPELL_DROP_OFFSET * SQRT_2 / SPELL_DROP_DELAY;

pub fn spawn_spell(
    world: &mut World,
    position: Vector2<f32>,
    action: Box<dyn Action>,
    drawable: Drawable,
) -> Entity {
    world.spawn((
        Position(position + Vector2::new(SPELL_DROP_OFFSET, -SPELL_DROP_OFFSET)),
        drawable,
        Mover {
            speed: SPELL_DROP_SPEED,
            target: position,
            arrived: false,
        },
        Delay {
            time_left: SPELL_DROP_DELAY,
        },
        OnDespawn(action),
    ))
}
