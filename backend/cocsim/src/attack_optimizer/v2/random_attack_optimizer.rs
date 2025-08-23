use std::collections::HashMap;

use rand_pcg::Pcg64Mcg;

use crate::{
    ValidatedMap,
    attack_optimizer::{
        Army,
        AttackPlanExecutionStats,
        execute_attack_plan,
        v2::AttackPlan,
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
    plans: HashMap<AttackPlan, AttackPlanExecutionStats>,
    plans_per_step: usize,
}

impl RandomAttackOptimizer {
    pub fn new(map: ValidatedMap, army: Army, plans_per_step: usize) -> Self {
        Self {
            map,
            army,
            rng: Pcg64Mcg::new(RNG_INITIAL_STATE),
            plans: HashMap::new(),
            plans_per_step,
        }
    }

    pub fn step(&mut self) {
        let mut repeats = 0;

        for _ in 0..self.plans_per_step {
            let new_plan = AttackPlan::new_randomized(&self.army, &self.map, &mut self.rng);

            if self.plans.contains_key(&new_plan) {
                repeats += 1;

                continue;
            }

            let new_stats = execute_attack_plan(
                &self.map,
                &new_plan.executor_actions(&self.army),
                ATTACK_PLAN_EXECUTIONS_COUNT,
                ATTACK_PLAN_EXECUTOR_TPS,
            );

            self.plans.insert(new_plan, new_stats);
        }

        println!("{repeats}");
    }

    pub fn plans(&self) -> &HashMap<AttackPlan, AttackPlanExecutionStats> {
        &self.plans
    }
}
