use rand_pcg::Pcg64Mcg;

use crate::{
    AttackOptimizer,
    AttackPlan,
    AttackPlanExecutionStats,
    SpellModelEnum,
    UnitModelEnum,
    ValidatedMap,
    WithCount,
    consts::{
        ATTACK_PLAN_EXECUTIONS_COUNT,
        ATTACK_PLAN_EXECUTOR_TPS,
        RNG_INITIAL_STATE,
    },
    execute_attack_plan,
};

pub struct RandomAttackOptimizer {
    map: ValidatedMap,
    units: Vec<WithCount<UnitModelEnum>>,
    spells: Vec<WithCount<SpellModelEnum>>,
    rng: Pcg64Mcg,
    plan: Option<(AttackPlan, AttackPlanExecutionStats)>,
    plans_per_step: usize,
}

impl RandomAttackOptimizer {
    pub fn new(
        map: ValidatedMap,
        units: Vec<WithCount<UnitModelEnum>>,
        spells: Vec<WithCount<SpellModelEnum>>,
        plans_per_step: usize,
    ) -> Self {
        Self {
            map,
            units,
            spells,
            rng: Pcg64Mcg::new(RNG_INITIAL_STATE),
            plan: None,
            plans_per_step,
        }
    }

    fn init_plan(&mut self) {
        if self.plan.is_none() {
            let plan = AttackPlan::new_randomized(&self.units, &self.spells, &mut self.rng);
            let stats = execute_attack_plan(
                &self.map,
                &plan,
                ATTACK_PLAN_EXECUTIONS_COUNT,
                ATTACK_PLAN_EXECUTOR_TPS,
            );

            self.plan = Some((plan, stats));
        }
    }
}

impl AttackOptimizer for RandomAttackOptimizer {
    fn best(&self) -> Option<&(AttackPlan, AttackPlanExecutionStats)> {
        self.plan.as_ref()
    }

    fn step(&mut self) -> &(AttackPlan, AttackPlanExecutionStats) {
        if self.plan.is_none() {
            self.init_plan();
        }

        let (plan, stats) = self.plan.as_mut().unwrap();

        for _ in 0..self.plans_per_step {
            let new_plan = AttackPlan::new_randomized(&self.units, &self.spells, &mut self.rng);
            let new_stats = execute_attack_plan(
                &self.map,
                &new_plan,
                ATTACK_PLAN_EXECUTIONS_COUNT,
                ATTACK_PLAN_EXECUTOR_TPS,
            );

            if new_stats.score >= stats.score {
                *stats = new_stats;
                *plan = new_plan;
            }
        }

        self.plan.as_ref().unwrap()
    }
}
