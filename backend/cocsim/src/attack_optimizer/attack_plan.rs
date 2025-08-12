use arbitrary::Arbitrary;
use rand::{
    Rng,
    seq::IndexedRandom,
};

use crate::{
    UnitWithCount,
    attack_optimizer::AttackPlanUnitGroup,
};

#[derive(Clone, Arbitrary, Debug)]
pub struct AttackPlan {
    pub units: Vec<AttackPlanUnitGroup>,
}

impl AttackPlan {
    pub fn new_randomized(units: &[UnitWithCount], rng: &mut impl Rng) -> Self {
        Self {
            units: units
                .iter()
                .map(|unit| AttackPlanUnitGroup::new_randomized(unit.unit.clone(), unit.count, rng))
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

        Self { units }
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

        Self { units }
    }
}
