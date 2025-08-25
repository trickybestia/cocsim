pub mod attack_optimizer;
pub mod buildings;
pub mod colliders;
pub mod consts;
mod game;
mod geometry;
mod map;
mod shape;
pub mod spells;
#[cfg(feature = "test-maps")]
pub mod test_maps;
pub mod units;
mod usize_with_max;
pub mod utils;
mod with_housing_space;

pub use game::Game;
pub use map::*;
pub use shape::{
    Shape,
    ShapeColor,
};
pub use usize_with_max::*;
pub use with_housing_space::*;
