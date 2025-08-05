use hecs::PreparedQuery;

use crate::{
    Game,
    game::features::to_be_deleted::ToBeDeleted,
    utils::AnyMapExt,
};

/// Despawn entity after time
pub struct Delay {
    pub time_left: f32,
}

pub fn update(game: &mut Game) {
    let mut to_be_deleted = Vec::new();

    for (id, delay) in game
        .cache
        .get_mut_or_default::<PreparedQuery<&mut Delay>>()
        .query_mut(&mut game.world)
    {
        delay.time_left = 0.0f32.max(delay.time_left - game.delta_time);

        if delay.time_left == 0.0 {
            to_be_deleted.push(id);
        }
    }

    for id in to_be_deleted {
        game.world.insert_one(id, ToBeDeleted).unwrap();
    }
}
