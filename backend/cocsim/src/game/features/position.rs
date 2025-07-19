use nalgebra::Vector2;
use shipyard::{
    Component,
    ViewMut,
    track::InsertionAndModification,
};

pub struct Position(pub Vector2<f32>);

impl Component for Position {
    type Tracking = InsertionAndModification;
}

pub fn cleanup_tracking(v_position: ViewMut<Position>) {
    v_position.clear_all_inserted_and_modified();
}
