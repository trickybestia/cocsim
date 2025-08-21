use nalgebra::Vector2;

use crate::{
    AttackPlan,
    AttackPlanSpell,
    Game,
    SpellModel,
    SpellModelEnum,
    UnitModelEnum,
    ValidatedMap,
    attack_optimizer::AttackPlanUnitGroup,
    consts::{
        SPELL_DROP_COOLDOWN,
        SPELL_GROUP_DROP_COOLDOWN,
        UNIT_DROP_COOLDOWN,
        UNIT_GROUP_DROP_COOLDOWN,
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

pub struct AttackPlanExecutor {
    actions: Vec<AttackPlanExecutorAction>,
}

impl AttackPlanExecutor {
    pub fn new(plan: &AttackPlan, map: &ValidatedMap) -> Self {
        let mut data = plan
            .units
            .iter()
            .cloned()
            .map(UnitGroupOrSpell::UnitGroup)
            .chain(plan.spells.iter().cloned().map(UnitGroupOrSpell::Spell))
            .collect::<Vec<_>>();

        // sort reversed by drop_time key
        data.sort_unstable_by(|a, b| b.drop_time().total_cmp(&a.drop_time()));

        let mut actions = Vec::new();
        let mut next_drop_time = 0.0f32;

        for unit_group_or_spell in data {
            match unit_group_or_spell {
                UnitGroupOrSpell::UnitGroup(unit_group) => {
                    let position = unit_group.cartesian_position(&map.map_size(), map.drop_zone());

                    next_drop_time = next_drop_time.max(unit_group.drop_time);

                    for _ in 0..unit_group.count {
                        actions.push(AttackPlanExecutorAction {
                            spawnable: Spawnable::Unit(unit_group.unit_model.clone()),
                            position,
                            drop_time: next_drop_time,
                        });

                        next_drop_time += UNIT_DROP_COOLDOWN;
                    }

                    next_drop_time += UNIT_GROUP_DROP_COOLDOWN;
                }
                UnitGroupOrSpell::Spell(spell) => {
                    let position = spell.cartesian_position(&map.map_size());

                    next_drop_time = next_drop_time.max(spell.drop_time);

                    for _ in 0..spell.count {
                        actions.push(AttackPlanExecutorAction {
                            spawnable: Spawnable::Spell(spell.spell_model.clone()),
                            position,
                            drop_time: next_drop_time,
                        });

                        next_drop_time += SPELL_DROP_COOLDOWN;
                    }

                    next_drop_time += SPELL_GROUP_DROP_COOLDOWN;
                }
            }
        }

        actions.reverse();

        Self { actions }
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
