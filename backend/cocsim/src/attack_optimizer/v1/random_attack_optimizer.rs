use rand_pcg::Pcg64Mcg;

use crate::{
    ValidatedMap,
    attack_optimizer::{
        Army,
        AttackPlanExecutionStats,
        execute_attack_plan,
        v1::{
            AttackOptimizer,
            AttackPlan,
        },
    },
    consts::{
        ATTACK_PLAN_EXECUTIONS_COUNT,
        ATTACK_PLAN_EXECUTOR_TPS,
        RNG_INITIAL_STATE,
    },
};

pub struct RandomAttackOptimizer {
    map: ValidatedMap,
    army: Army,
    rng: Pcg64Mcg,
    plan: Option<(AttackPlan, AttackPlanExecutionStats)>,
    plans_per_step: usize,
}

impl RandomAttackOptimizer {
    pub fn new(map: ValidatedMap, army: Army, plans_per_step: usize) -> Self {
        Self {
            map,
            army,
            rng: Pcg64Mcg::new(RNG_INITIAL_STATE),
            plan: None,
            plans_per_step,
        }
    }

    fn init_plan(&mut self) {
        if self.plan.is_none() {
            let plan = AttackPlan::new_randomized(&self.army, &mut self.rng);
            let stats = execute_attack_plan(
                &self.map,
                &plan.executor_actions(&self.map),
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
            let new_plan = AttackPlan::new_randomized(&self.army, &mut self.rng);
            let new_stats = execute_attack_plan(
                &self.map,
                &new_plan.executor_actions(&self.map),
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
