use hecs::Entity;

use crate::{
    Game,
    game::features::{
        actions::Action,
        attack::Team,
        delay::Delay,
        position::Position,
        to_be_despawned::OnDespawn,
    },
};

#[derive(Debug, Clone)]
pub struct WithDelay {
    pub time: f32,
    pub action: Box<dyn Action>,
}

impl Action for WithDelay {
    fn call(&self, actor: Entity, game: &mut Game) {
        let id = game.world.spawn((
            Delay {
                time_left: self.time,
            },
            OnDespawn(self.action.clone()),
        ));

        let team = game.world.get::<&Team>(actor).map(|team| *team);

        if let Ok(team) = team {
            game.world.insert_one(id, team).unwrap();
        }

        let position = game.world.get::<&Position>(actor).map(|position| *position);

        if let Ok(position) = position {
            game.world.insert_one(id, position).unwrap();
        }
    }
}
