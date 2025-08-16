use nalgebra::Vector2;

use crate::{
    AttackPlan,
    AttackPlanSpell,
    Game,
    SpellModel,
    SpellModelEnum,
    UnitModelEnum,
    attack_optimizer::AttackPlanUnitGroup,
    consts::{
        SPELL_DROP_COOLDOWN,
        UNIT_DROP_COOLDOWN,
        UNIT_DROP_GROUP_COOLDOWN,
    },
};

enum Spawnable {
    Unit(UnitModelEnum),
    Spell(SpellModelEnum),
}

struct AttackPlanExecutorAction {
    pub spawnable: Spawnable,
    pub position: Vector2<f32>,
    pub drop_time: f32,
}

enum UnitGroupOrSpell {
    UnitGroup(AttackPlanUnitGroup),
    Spell(AttackPlanSpell),
}

impl UnitGroupOrSpell {
    pub fn drop_time(&self) -> f32 {
        match self {
            UnitGroupOrSpell::UnitGroup(unit_group) => unit_group.drop_time,
            UnitGroupOrSpell::Spell(spell) => spell.drop_time,
        }
    }
}

enum AttackPlanExecutorState {
    Created(Vec<UnitGroupOrSpell>),
    // Can't access DropZone and MapSize in AttackPlanExecutor::new, moving initialization to
    // first AttackPlanExecutor::tick call
    Initialized {
        actions: Vec<AttackPlanExecutorAction>,
    },
}

pub struct AttackPlanExecutor {
    state: AttackPlanExecutorState,
}

impl AttackPlanExecutor {
    pub fn new(plan: &AttackPlan) -> Self {
        let mut data = plan
            .units
            .iter()
            .cloned()
            .map(UnitGroupOrSpell::UnitGroup)
            .chain(plan.spells.iter().cloned().map(UnitGroupOrSpell::Spell))
            .collect::<Vec<_>>();

        // sort reversed by drop_time key
        data.sort_unstable_by(|a, b| b.drop_time().total_cmp(&a.drop_time()));

        Self {
            state: AttackPlanExecutorState::Created(data),
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.state {
            AttackPlanExecutorState::Created(unit_group_or_spells) => {
                unit_group_or_spells.is_empty()
            }
            AttackPlanExecutorState::Initialized { actions } => actions.is_empty(),
        }
    }

    pub fn tick(&mut self, game: &mut Game) {
        if let AttackPlanExecutorState::Created(data) = &mut self.state {
            let mut actions = Vec::new();
            let mut next_drop_time = 0.0f32;

            for unit_group_or_spell in data {
                match unit_group_or_spell {
                    UnitGroupOrSpell::UnitGroup(unit_group) => {
                        let position =
                            unit_group.cartesian_position(&game.map_size, &game.drop_zone.0);

                        next_drop_time = next_drop_time.max(unit_group.drop_time);

                        for _ in 0..unit_group.count {
                            actions.push(AttackPlanExecutorAction {
                                spawnable: Spawnable::Unit(unit_group.unit_model.clone()),
                                position,
                                drop_time: next_drop_time,
                            });

                            next_drop_time += UNIT_DROP_COOLDOWN;
                        }

                        next_drop_time += UNIT_DROP_GROUP_COOLDOWN;
                    }
                    UnitGroupOrSpell::Spell(spell) => {
                        let position = spell.cartesian_position(&game.map_size);

                        next_drop_time = next_drop_time.max(spell.drop_time);

                        actions.push(AttackPlanExecutorAction {
                            spawnable: Spawnable::Spell(spell.spell_model.clone()),
                            position,
                            drop_time: next_drop_time,
                        });

                        next_drop_time += SPELL_DROP_COOLDOWN;
                    }
                }
            }

            actions.reverse();

            self.state = AttackPlanExecutorState::Initialized { actions }
        }

        if let AttackPlanExecutorState::Initialized { actions } = &mut self.state {
            while !actions.is_empty() && actions.last().unwrap().drop_time <= game.time_elapsed() {
                let action = actions.pop().unwrap();

                match action.spawnable {
                    Spawnable::Unit(unit_model_enum) => {
                        game.spawn_attack_unit(&unit_model_enum, action.position)
                    }
                    Spawnable::Spell(spell_model_enum) => {
                        spell_model_enum.spawn(game, action.position)
                    }
                }
            }
        } else {
            unreachable!();
        }
    }
}
