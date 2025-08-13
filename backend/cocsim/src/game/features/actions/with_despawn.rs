use hecs::Entity;

use crate::{
    Game,
    game::features::{
        actions::{
            Action,
            ActionEnum,
        },
        to_be_despawned::ToBeDespawned,
    },
};

#[derive(Clone, Debug)]
pub struct WithDespawn(pub Box<ActionEnum>);

impl Action for WithDespawn {
    fn call(&self, actor: Entity, game: &mut Game) {
        self.0.call(actor, game);

        game.world.insert_one(actor, ToBeDespawned).unwrap();
    }
}
