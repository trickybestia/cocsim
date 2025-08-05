use crate::Game;

pub mod air_unit;
pub mod building;

pub struct FindTargetRequest;

pub fn handle_find_target_requests(game: &mut Game) {
    air_unit::handle_find_target_requests(game);
    building::handle_find_target_requests(game);

    for id in game
        .world
        .query_mut::<&FindTargetRequest>()
        .into_iter()
        .map(|(id, _)| id)
        .collect::<Vec<_>>()
    {
        game.world.remove_one::<FindTargetRequest>(id).unwrap();
    }
}
