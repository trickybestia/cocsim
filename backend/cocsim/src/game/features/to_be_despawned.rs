use std::sync::Arc;

use hecs::{
    Entity,
    PreparedQuery,
    With,
};

use crate::{
    Game,
    game::features::actions::Action,
    utils::AnyMapExt,
};

pub struct ToBeDespawned;

pub struct OnDespawn(pub Arc<dyn Action>);

pub fn handle_to_be_despawned(game: &mut Game) {
    for (id, action) in create_on_despawn_queue(game) {
        action.call(id, game);
    }

    for id in create_despawn_queue(game) {
        game.world.despawn(id).unwrap();
    }
}

fn create_on_despawn_queue(game: &mut Game) -> Vec<(Entity, Arc<dyn Action>)> {
    game.cache
        .get_mut_or_default::<PreparedQuery<With<&OnDespawn, &ToBeDespawned>>>()
        .query_mut(&mut game.world)
        .into_iter()
        .map(|(id, on_despawn)| (id, on_despawn.0.clone()))
        .collect::<Vec<_>>()
}

fn create_despawn_queue(game: &mut Game) -> Vec<Entity> {
    game.cache
        .get_mut_or_default::<PreparedQuery<With<(), &ToBeDespawned>>>()
        .query_mut(&mut game.world)
        .into_iter()
        .map(|(id, _)| id)
        .collect::<Vec<_>>()
}
