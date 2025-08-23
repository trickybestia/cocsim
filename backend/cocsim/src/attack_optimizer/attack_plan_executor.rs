use nalgebra::Vector2;

use crate::{
    Game,
    SpellModel,
    SpellModelEnum,
    UnitModelEnum,
};

#[derive(Clone)]
pub enum Spawnable {
    Unit(UnitModelEnum),
    Spell(SpellModelEnum),
}

#[derive(Clone)]
pub struct AttackPlanExecutorAction {
    pub spawnable: Spawnable,
    pub position: Vector2<f32>,
    pub drop_time: f32,
}

pub struct AttackPlanExecutor {
    actions: Vec<AttackPlanExecutorAction>,
}

impl AttackPlanExecutor {
    pub fn new(actions: &[AttackPlanExecutorAction]) -> Self {
        Self {
            actions: actions.to_owned(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    pub fn tick(&mut self, game: &mut Game) {
        while !self.actions.is_empty()
            && self.actions.last().unwrap().drop_time <= game.time_elapsed()
        {
            let action = self.actions.pop().unwrap();

            match action.spawnable {
                Spawnable::Unit(unit_model_enum) => {
                    game.spawn_attack_unit(&unit_model_enum, action.position)
                }
                Spawnable::Spell(spell_model_enum) => spell_model_enum.spawn(game, action.position),
            }
        }
    }
}
