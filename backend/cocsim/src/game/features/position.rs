use nalgebra::Vector2;
use shipyard::Component;

#[derive(Component)]
pub struct Position(pub Vector2<f32>);
