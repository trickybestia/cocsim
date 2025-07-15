use shipyard::{
    Unique,
    UniqueViewMut,
};

use crate::consts::*;

#[derive(Unique)]
pub struct Time {
    pub elapsed: f32,
    pub delta: f32,
}

pub fn set_delta_time(value: f32, mut time: UniqueViewMut<Time>) {
    time.delta = value;
}

pub fn update_elapsed_time(mut time: UniqueViewMut<Time>) {
    time.elapsed = MAX_ATTACK_DURATION.min(time.elapsed + time.delta);
}
