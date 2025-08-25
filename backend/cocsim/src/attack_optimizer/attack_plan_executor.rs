use nalgebra::Vector2;

use crate::{
    Game,
    WithCount,
    consts::{
        SPELL_DROP_COOLDOWN,
        SPELL_GROUP_DROP_COOLDOWN,
        UNIT_DROP_COOLDOWN,
        UNIT_GROUP_DROP_COOLDOWN,
    },
    spells::{
        SpellModel,
        SpellModelEnum,
    },
    units::UnitModelEnum,
};

#[derive(Clone)]
enum SingleSpawnable {
    Unit(UnitModelEnum),
    Spell(SpellModelEnum),
}

#[derive(Clone)]
struct AttackPlanExecutorActionInternal {
    pub spawnable: SingleSpawnable,
    pub position: Vector2<f32>,
    pub drop_time: f32,
}

#[derive(Clone)]
pub enum Spawnable {
    UnitGroup(WithCount<UnitModelEnum>),
    SpellGroup(WithCount<SpellModelEnum>),
}

#[derive(Clone)]
pub struct AttackPlanExecutorAction {
    pub spawnable: Spawnable,
    pub position: Vector2<f32>,
    pub drop_time: f32,
}

pub struct AttackPlanExecutor {
    actions: Vec<AttackPlanExecutorActionInternal>,
}

impl AttackPlanExecutor {
    pub fn new(mut actions: Vec<AttackPlanExecutorAction>) -> Self {
        // sort reversed by drop_time key
        actions.sort_unstable_by(|a, b| b.drop_time.total_cmp(&a.drop_time));

        let mut result = Vec::new();
        let mut next_drop_time = 0.0f32;

        for action in actions {
            match action.spawnable {
                Spawnable::UnitGroup(unit_group) => {
                    next_drop_time = next_drop_time.max(action.drop_time);

                    for _ in 0..unit_group.count {
                        result.push(AttackPlanExecutorActionInternal {
                            spawnable: SingleSpawnable::Unit(unit_group.value.clone()),
                            position: action.position,
                            drop_time: next_drop_time,
                        });

                        next_drop_time += UNIT_DROP_COOLDOWN;
                    }

                    next_drop_time += UNIT_GROUP_DROP_COOLDOWN;
                }
                Spawnable::SpellGroup(spell_group) => {
                    next_drop_time = next_drop_time.max(action.drop_time);

                    for _ in 0..spell_group.count {
                        result.push(AttackPlanExecutorActionInternal {
                            spawnable: SingleSpawnable::Spell(spell_group.value.clone()),
                            position: action.position,
                            drop_time: next_drop_time,
                        });

                        next_drop_time += SPELL_DROP_COOLDOWN;
                    }

                    next_drop_time += SPELL_GROUP_DROP_COOLDOWN;
                }
            }
        }

        result.reverse();

        Self { actions: result }
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
                SingleSpawnable::Unit(unit_model_enum) => {
                    game.spawn_attack_unit(&unit_model_enum, action.position)
                }
                SingleSpawnable::Spell(spell_model_enum) => {
                    spell_model_enum.spawn(game, action.position)
                }
            }
        }
    }
}
