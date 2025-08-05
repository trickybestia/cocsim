use hecs::{
    Entity,
    PreparedQuery,
    With,
};

use crate::{
    Game,
    game::features::actions::{
        Action,
        ActionEnum,
    },
    utils::AnyMapExt,
};

pub struct ToBeDeleted;

pub struct OnDelete(pub ActionEnum);

pub fn handle_to_be_deleted(game: &mut Game) {
    for (id, action) in create_on_delete_queue(game) {
        action.call(id, game);
    }

    for id in create_despawn_queue(game) {
        game.world.despawn(id).unwrap();
    }
}

fn create_on_delete_queue(game: &mut Game) -> Vec<(Entity, ActionEnum)> {
    game.cache
        .get_mut_or_default::<PreparedQuery<With<&OnDelete, &ToBeDeleted>>>()
        .query_mut(&mut game.world)
        .into_iter()
        .map(|(id, on_delete)| (id, on_delete.0.clone()))
        .collect::<Vec<_>>()
}

fn create_despawn_queue(game: &mut Game) -> Vec<Entity> {
    game.cache
        .get_mut_or_default::<PreparedQuery<With<(), &ToBeDeleted>>>()
        .query_mut(&mut game.world)
        .into_iter()
        .map(|(id, _)| id)
        .collect::<Vec<_>>()
}
