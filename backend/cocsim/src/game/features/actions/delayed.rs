use std::ops::Deref;

use shipyard::{
    AllStoragesViewMut,
    EntityId,
    Get,
    View,
};

use crate::game::features::{
    actions::{
        Action,
        ActionEnum,
    },
    attack::Team,
    delay::Delay,
    position::Position,
    to_be_deleted::OnDelete,
};

#[derive(Clone, Debug)]
pub struct Delayed {
    pub time: f32,
    pub action: Box<ActionEnum>,
}

impl Action for Delayed {
    fn call(&self, actor: EntityId, all_storages: &mut AllStoragesViewMut) {
        let id = all_storages.add_entity((
            Delay {
                time_left: self.time,
            },
            OnDelete(self.action.deref().clone()),
        ));

        let team = all_storages
            .borrow::<View<Team>>()
            .unwrap()
            .get(actor)
            .map(|team| *team);

        if let Ok(team) = team {
            all_storages.add_component(id, (team,));
        }

        let position = all_storages
            .borrow::<View<Position>>()
            .unwrap()
            .get(actor)
            .map(|position| position.0);

        if let Ok(position) = position {
            all_storages.add_component(id, (Position(position),));
        }
    }
}
