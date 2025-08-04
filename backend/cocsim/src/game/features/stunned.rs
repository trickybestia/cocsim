use shipyard::{
    Component,
    ViewMut,
};

#[derive(Component)]
pub struct Stunned;

pub fn clear(mut v_stunned: ViewMut<Stunned>) {
    v_stunned.clear();
}
