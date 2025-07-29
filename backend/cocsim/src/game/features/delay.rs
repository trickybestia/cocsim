use shipyard::{
    AddComponent,
    Component,
    IntoIter,
    UniqueView,
    ViewMut,
};

use crate::game::features::{
    time::Time,
    to_be_deleted::ToBeDeleted,
};

/// Despawn entity after time
#[derive(Component)]
pub struct Delay {
    pub time_left: f32,
}

pub fn update(
    time: UniqueView<Time>,
    mut v_delay: ViewMut<Delay>,
    mut v_to_be_deleted: ViewMut<ToBeDeleted>,
) {
    for (id, delay) in (&mut v_delay).iter().with_id() {
        delay.time_left = 0.0f32.max(delay.time_left - time.delta);

        if delay.time_left == 0.0 {
            v_to_be_deleted.add_component_unchecked(id, ToBeDeleted);
        }
    }
}
