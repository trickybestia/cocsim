use arbitrary::Arbitrary;
use rand::{
    Rng,
    seq::IndexedRandom,
};

use crate::{
    ValidatedMap,
    WithCount,
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
        let mut result = Vec::new();

        for unit_group in &self.units {
            let position = unit_group.cartesian_position(&map.map_size(), map.drop_zone());

            result.push(AttackPlanExecutorAction {
                spawnable: Spawnable::UnitGroup(WithCount {
                    value: unit_group.unit_model.clone(),
                    count: unit_group.count,
                }),
                position,
                drop_time: unit_group.drop_time,
            });
        }

        for spell_group in &self.spells {
            let position = spell_group.cartesian_position(&map.map_size());

            result.push(AttackPlanExecutorAction {
                spawnable: Spawnable::SpellGroup(WithCount {
                    value: spell_group.spell_model.clone(),
                    count: spell_group.count,
                }),
                position,
                drop_time: spell_group.drop_time,
            });
        }

        result
    }
}
