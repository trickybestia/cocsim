use nalgebra::Vector2;
use shipyard::{
    Component,
    ViewMut,
};

#[derive(Component)]
#[track(All)]
pub struct Position(pub Vector2<f32>);

pub fn cleanup_tracking(mut v_position: ViewMut<Position>) {
    v_position.clear_all_removed_and_deleted();
    v_position.clear_all_inserted_and_modified();
}
