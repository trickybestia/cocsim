use hecs::{
    PreparedQuery,
    Without,
};
use nalgebra::Vector2;

use crate::{
    Game,
    consts::*,
    game::features::{
        position::Position,
        stunned::Stunned,
    },
    utils::AnyMapExt,
};

pub struct Mover {
    pub speed: f32,
    pub target: Vector2<f32>,
    pub arrived: bool,
}

pub fn r#move(game: &mut Game) {
    for (_id, (position, mover)) in game
        .cache
        .get_mut_or_default::<PreparedQuery<Without<(&mut Position, &mut Mover), &Stunned>>>()
        .query_mut(&mut game.world)
    {
        mover.arrived = position.0.metric_distance(&mover.target) <= UNIT_DISTANCE_TO_WAYPOINT_EPS;

        if !mover.arrived {
            let direction = (mover.target - position.0).normalize();

            position.0 += direction * mover.speed * game.delta_time;
        }
    }
}
