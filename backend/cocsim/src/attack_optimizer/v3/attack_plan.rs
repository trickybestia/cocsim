use gomez::{
    Domain,
    Function,
    Problem,
    nalgebra::{
        Dyn,
        IsContiguous,
        Storage,
        Vector,
    },
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
        execute_attack_plan,
        v3::{
            AttackPlanSpellGroup,
            attack_plan_unit_group::AttackPlanUnitGroup,
        },
    },
    consts::{
        ATTACK_PLAN_EXECUTIONS_COUNT,
        ATTACK_PLAN_EXECUTOR_TPS,
    },
};

#[derive(Clone, Debug)]
pub struct AttackPlan {
    pub map: ValidatedMap,
    pub army: Army,
}

impl AttackPlan {
    pub fn executor_actions(
        &self,
        mut x: impl Iterator<Item = f32>,
    ) -> Vec<AttackPlanExecutorAction> {
        let mut result = Vec::new();

        for unit_with_count in &self.army.units {
            let unit_group = AttackPlanUnitGroup::from_numbers(&mut x);

            let position = unit_group.cartesian_position(&self.map.size(), self.map.drop_zone());

            result.push(AttackPlanExecutorAction {
                spawnable: Spawnable::UnitGroup(WithCount {
                    value: unit_with_count.value.clone(),
                    count: unit_with_count.count,
                }),
                position,
                drop_time: unit_group.drop_time,
            });
        }

        for spell_with_count in &self.army.spells {
            let spell_group = AttackPlanSpellGroup::from_numbers(&mut x);

            let position = spell_group.cartesian_position(&self.map.size());

            result.push(AttackPlanExecutorAction {
                spawnable: Spawnable::SpellGroup(WithCount {
                    value: spell_with_count.value.clone(),
                    count: spell_with_count.count,
                }),
                position,
                drop_time: spell_group.drop_time,
            });
        }

        result
    }
}

impl Problem for AttackPlan {
    type Field = f32;

    fn domain(&self) -> Domain<Self::Field> {
        let mut lower = Vec::new();
        let mut upper = Vec::new();

        for _ in &self.army.units {
            for (lower_bound, upper_bound) in AttackPlanUnitGroup::domain() {
                lower.push(lower_bound);
                upper.push(upper_bound);
            }
        }

        for _ in &self.army.spells {
            for (lower_bound, upper_bound) in AttackPlanSpellGroup::domain() {
                lower.push(lower_bound);
                upper.push(upper_bound);
            }
        }

        Domain::rect(lower, upper)
    }
}

impl Function for AttackPlan {
    fn apply<Sx>(&self, x: &Vector<Self::Field, Dyn, Sx>) -> Self::Field
    where
        Sx: Storage<Self::Field, Dyn> + IsContiguous,
    {
        -execute_attack_plan(
            &self.map,
            &self.executor_actions(x.iter().cloned()),
            ATTACK_PLAN_EXECUTIONS_COUNT,
            ATTACK_PLAN_EXECUTOR_TPS,
        )
        .score
    }
}
