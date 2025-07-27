use arbitrary::Arbitrary;
use rand::{
    Rng,
    seq::IndexedRandom,
};

use crate::{
    UnitModelEnum,
    attack_optimizer::AttackPlanUnit,
};

#[derive(Clone, Arbitrary)]
pub struct AttackPlan {
    units: Vec<AttackPlanUnit>,
}

impl AttackPlan {
    pub fn new_randomized(units: &[UnitModelEnum], rng: &mut impl Rng) -> Self {
        Self {
            units: units
                .iter()
                .map(|unit_model| AttackPlanUnit::new_randomized(unit_model.clone(), rng))
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

    pub fn mutate(&self, rng: &mut impl Rng) -> Self {
        let units = self.units.iter().map(|unit| unit.mutate(rng)).collect();

        Self { units }
    }

    pub fn units(&self) -> &[AttackPlanUnit] {
        &self.units
    }
}
