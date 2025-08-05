use hecs::Entity;

use crate::Game;

pub struct Stunned;

pub fn clear(game: &mut Game) {
    let stunned = game
        .world
        .query_mut::<&Stunned>()
        .into_iter()
        .map(|(id, _)| id)
        .collect::<Vec<Entity>>();

    for id in stunned {
        game.world.remove_one::<Stunned>(id).unwrap();
    }
}
