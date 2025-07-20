use nalgebra::Vector2;
use shipyard::Component;

#[derive(Component)]
pub struct WaypointMover {
    pub speed: f32,
    pub waypoints: Vec<Vector2<f32>>,
}

impl WaypointMover {
    pub fn arrived(&self) -> bool {
        self.waypoints.is_empty()
    }
}
