use nalgebra::Vector2;
use shipyard::{
    Component,
    IntoIter,
    UniqueView,
    View,
    ViewMut,
};

use crate::{
    consts::*,
    game::features::{
        position::Position,
        stunned::Stunned,
        time::Time,
    },
};

#[derive(Component)]
pub struct WaypointMover {
    pub speed: f32,
    /// waypoints[0] - last waypoint (farthest), waypoints[waypoints.len() - 1]
    /// - first waypoint (nearest)
    pub waypoints: Vec<Vector2<f32>>,
}

pub fn r#move(
    time: UniqueView<Time>,
    v_stunned: View<Stunned>,
    mut v_position: ViewMut<Position>,
    mut v_waypoint_mover: ViewMut<WaypointMover>,
) {
    for (position, waypoint_mover, _) in
        (&mut v_position, &mut v_waypoint_mover, !&v_stunned).iter()
    {
        if waypoint_mover.waypoints.is_empty() {
            continue;
        }

        let next_waypoint = waypoint_mover.waypoints.last().unwrap();

        if position.0.metric_distance(next_waypoint) <= UNIT_DISTANCE_TO_WAYPOINT_EPS {
            waypoint_mover.waypoints.pop().unwrap();
        } else {
            let direction = (next_waypoint - position.0).normalize();

            position.0 += direction * waypoint_mover.speed * time.delta;
        }
    }
}
