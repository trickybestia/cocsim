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

pub struct WaypointMover {
    pub speed: f32,
    /// waypoints[0] - last waypoint (farthest), waypoints[waypoints.len() - 1]
    /// - first waypoint (nearest)
    pub waypoints: Vec<Vector2<f32>>,
}

pub fn r#move(game: &mut Game) {
    for (_id, (position, waypoint_mover)) in game.cache.get_mut_or_default::<PreparedQuery<Without<(&mut Position, &mut WaypointMover), &Stunned>>>().query_mut(&mut game.world)
    {
        if waypoint_mover.waypoints.is_empty() {
            continue;
        }

        let next_waypoint = waypoint_mover.waypoints.last().unwrap();

        if position.0.metric_distance(next_waypoint) <= UNIT_DISTANCE_TO_WAYPOINT_EPS {
            waypoint_mover.waypoints.pop().unwrap();
        } else {
            let direction = (next_waypoint - position.0).normalize();

            position.0 += direction * waypoint_mover.speed * game.delta_time;
        }
    }
}
