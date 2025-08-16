use std::iter::repeat;

use arbitrary::Arbitrary;
use rand::{
    Rng,
    seq::IndexedRandom,
};

use crate::{
    SpellWithCount,
    UnitWithCount,
    attack_optimizer::{
        AttackPlanUnitGroup,
        attack_plan_spell::AttackPlanSpell,
    },
};

#[derive(Clone, Arbitrary, Debug)]
pub struct AttackPlan {
    pub units: Vec<AttackPlanUnitGroup>,
    pub spells: Vec<AttackPlanSpell>,
}

impl AttackPlan {
    pub fn new_randomized(
        units: &[UnitWithCount],
        spells: &[SpellWithCount],
        rng: &mut impl Rng,
    ) -> Self {
        Self {
            units: units
                .iter()
                .map(|unit| AttackPlanUnitGroup::new_randomized(unit.unit.clone(), unit.count, rng))
                .collect(),
            spells: spells
                .iter()
                .map(|spell| repeat(spell.spell.clone()).take(spell.count))
                .flatten()
                .map(|spell| AttackPlanSpell::new_randomized(spell, rng))
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

    pub fn mutate(&self, rng: &mut impl Rng, probability: f64) -> Self {
        let units = self
            .units
            .iter()
            .map(|unit| {
                if rng.random_bool(probability) {
                    unit.mutate(rng)
                } else {
                    unit.clone()
                }
            })
            .collect();
        let spells = self
            .spells
            .iter()
            .map(|spell| {
                if rng.random_bool(probability) {
                    spell.mutate(rng)
                } else {
                    spell.clone()
                }
            })
            .collect();

        Self { units, spells }
    }
}
