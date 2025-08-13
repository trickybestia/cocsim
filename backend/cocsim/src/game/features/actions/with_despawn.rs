use std::sync::Arc;

use hecs::Entity;

use crate::{
    Game,
    game::features::{
        actions::Action,
        to_be_despawned::ToBeDespawned,
    },
};

#[derive(Debug)]
pub struct WithDespawn(pub Arc<dyn Action>);

impl Action for WithDespawn {
    fn call(&self, actor: Entity, game: &mut Game) {
        self.0.call(actor, game);

        game.world.insert_one(actor, ToBeDespawned).unwrap();
    }
}
