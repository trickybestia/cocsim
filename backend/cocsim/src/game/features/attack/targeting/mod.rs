use crate::Game;

pub mod air_unit;
pub mod building;

pub fn handle_retarget(game: &mut Game) {
    air_unit::handle_retarget(game);
    building::handle_retarget(game);
}
