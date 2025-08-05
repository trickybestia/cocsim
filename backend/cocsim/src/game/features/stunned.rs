use hecs::{
    Entity,
    PreparedQuery,
};

use crate::{
    Game,
    utils::AnyMapExt,
};

pub struct Stunned;

pub fn clear(game: &mut Game) {
    let stunned = game
        .cache
        .get_mut_or_default::<PreparedQuery<&Stunned>>()
        .query_mut(&mut game.world)
        .into_iter()
        .map(|(id, _)| id)
        .collect::<Vec<Entity>>();

    for id in stunned {
        game.world.remove_one::<Stunned>(id).unwrap();
    }
}
