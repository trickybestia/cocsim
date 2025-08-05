use hecs::{
    Entity,
    With,
};

use crate::{
    Game,
    game::features::actions::{
        Action,
        ActionEnum,
    },
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
    game.world
        .query_mut::<With<&OnDelete, &ToBeDeleted>>()
        .into_iter()
        .map(|(id, on_delete)| (id, on_delete.0.clone()))
        .collect::<Vec<_>>()
}

fn create_despawn_queue(game: &mut Game) -> Vec<Entity> {
    game.world
        .query_mut::<With<(), &ToBeDeleted>>()
        .into_iter()
        .map(|(id, _)| id)
        .collect::<Vec<_>>()
}
