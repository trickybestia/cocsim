use hecs::Entity;

use crate::{
    Game,
    game::features::actions::Action,
};

#[derive(Clone, Debug)]
pub struct EmptyAction;

impl Action for EmptyAction {
    fn call(&self, _actor: Entity, _game: &mut Game) {}
}
