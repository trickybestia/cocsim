use crate::Game;

pub mod air_unit;
pub mod building;

pub fn update(game: &mut Game) {
    air_unit::update(game);
    building::update(game);
}
