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
                .map(|spell| AttackPlanSpell::new_randomized(spell.spell.clone(), spell.count, rng))
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
}
