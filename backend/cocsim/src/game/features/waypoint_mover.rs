use nalgebra::Vector2;

pub struct WaypointMover {
    pub speed: f32,
    pub waypoints: Vec<Vector2<f32>>,
}
