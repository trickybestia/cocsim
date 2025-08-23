use arbitrary::Arbitrary;
use rand::{
    Rng,
    seq::IndexedRandom,
};

use crate::{
    ValidatedMap,
    attack_optimizer::{
        Army,
        attack_plan_executor::{
            AttackPlanExecutorAction,
            Spawnable,
        },
        v1::{
            AttackPlanSpellGroup,
            AttackPlanUnitGroup,
        },
    },
    consts::{
        SPELL_DROP_COOLDOWN,
        SPELL_GROUP_DROP_COOLDOWN,
        UNIT_DROP_COOLDOWN,
        UNIT_GROUP_DROP_COOLDOWN,
    },
};

#[derive(Clone, Arbitrary, Debug)]
pub struct AttackPlan {
    pub units: Vec<AttackPlanUnitGroup>,
    pub spells: Vec<AttackPlanSpellGroup>,
}

impl AttackPlan {
    pub fn new_randomized(army: &Army, rng: &mut impl Rng) -> Self {
        Self {
            units: army
                .units
                .iter()
                .map(|unit| {
                    AttackPlanUnitGroup::new_randomized(unit.value.clone(), unit.count, rng)
                })
                .collect(),
            spells: army
                .spells
                .iter()
                .map(|spell| {
                    AttackPlanSpellGroup::new_randomized(spell.value.clone(), spell.count, rng)
                })
                .collect(),
        }
    }

    pub fn merge(a: &Self, b: &Self, rng: &mut impl Rng) -> Self {
        let units = a
            .units
            .iter()
            .zip(b.units.iter())
            .map(|(a, b)| (*[a, b].choose(rng).unwrap()).clone())
            .collect();
        let spells = a
            .spells
            .iter()
            .zip(b.spells.iter())
            .map(|(a, b)| (*[a, b].choose(rng).unwrap()).clone())
            .collect();

        Self { units, spells }
    }

    pub fn mutate(&self, rng: &mut impl Rng, temperature: f32) -> Self {
        let units = self
            .units
            .iter()
            .map(|unit| unit.mutate(rng, temperature))
            .collect();
        let spells = self
            .spells
            .iter()
            .map(|spell| spell.mutate(rng, temperature))
            .collect();

        Self { units, spells }
    }

    pub fn executor_actions(&self, map: &ValidatedMap) -> Vec<AttackPlanExecutorAction> {
        enum Group {
            UnitGroup(AttackPlanUnitGroup),
            SpellGroup(AttackPlanSpellGroup),
        }

        impl Group {
            pub fn drop_time(&self) -> f32 {
                match self {
                    Group::UnitGroup(unit_group) => unit_group.drop_time,
                    Group::SpellGroup(spell_group) => spell_group.drop_time,
                }
            }
        }

        let mut groups = self
            .units
            .iter()
            .cloned()
            .map(Group::UnitGroup)
            .chain(self.spells.iter().cloned().map(Group::SpellGroup))
            .collect::<Vec<_>>();

        // sort reversed by drop_time key
        groups.sort_unstable_by(|a, b| b.drop_time().total_cmp(&a.drop_time()));

        let mut result = Vec::new();
        let mut next_drop_time = 0.0f32;

        for unit_group_or_spell in groups {
            match unit_group_or_spell {
                Group::UnitGroup(unit_group) => {
                    let position = unit_group.cartesian_position(&map.map_size(), map.drop_zone());

                    next_drop_time = next_drop_time.max(unit_group.drop_time);

                    for _ in 0..unit_group.count {
                        result.push(AttackPlanExecutorAction {
                            spawnable: Spawnable::Unit(unit_group.unit_model.clone()),
                            position,
                            drop_time: next_drop_time,
                        });

                        next_drop_time += UNIT_DROP_COOLDOWN;
                    }

                    next_drop_time += UNIT_GROUP_DROP_COOLDOWN;
                }
                Group::SpellGroup(spell) => {
                    let position = spell.cartesian_position(&map.map_size());

                    next_drop_time = next_drop_time.max(spell.drop_time);

                    for _ in 0..spell.count {
                        result.push(AttackPlanExecutorAction {
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

        result.reverse();

        result
    }
}
