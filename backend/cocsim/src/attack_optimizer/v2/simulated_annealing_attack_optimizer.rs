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

pub struct SimulatedAnnealingAttackOptimizer {
    map: ValidatedMap,
    army: Army,
    rng: Pcg64Mcg,
    plan: Option<(AttackPlan, AttackPlanExecutionStats)>,
    current_iteration: usize,
    last_new_found_iteration: usize,
}

impl SimulatedAnnealingAttackOptimizer {
    pub fn new(
        map: ValidatedMap,
        army: Army,
        initial_plan: Option<(AttackPlan, AttackPlanExecutionStats)>,
    ) -> Self {
        Self {
            map,
            army,
            rng: Pcg64Mcg::new(RNG_INITIAL_STATE),
            plan: initial_plan,
            current_iteration: 0,
            last_new_found_iteration: 0,
        }
    }

    pub fn step(&mut self, iterations: usize) -> &(AttackPlan, AttackPlanExecutionStats) {
        if self.plan.is_none() {
            self.init_plan();
        }

        let (plan, stats) = self.plan.as_mut().unwrap();

        for _ in 0..iterations {
            let new_plan = plan.random_neighbor(
                &self.map,
                Self::radius(self.current_iteration, self.last_new_found_iteration),
                &mut self.rng,
            );

            let new_stats = execute_attack_plan(
                &self.map,
                &new_plan.executor_actions(&self.army),
                ATTACK_PLAN_EXECUTIONS_COUNT,
                ATTACK_PLAN_EXECUTOR_TPS,
            );

            if new_stats.score >= stats.score {
                *stats = new_stats;
                *plan = new_plan;

                self.last_new_found_iteration = self.current_iteration;
            }

            self.current_iteration += 1;
        }

        self.plan.as_ref().unwrap()
    }

    pub fn best(&self) -> Option<&(AttackPlan, AttackPlanExecutionStats)> {
        self.plan.as_ref()
    }

    pub fn iterations_since_last_new_found(&self) -> usize {
        self.current_iteration - self.last_new_found_iteration
    }

    fn radius(current_iteration: usize, last_new_found_iteration: usize) -> usize {
        let iterations_delta = current_iteration - last_new_found_iteration;

        return iterations_delta / 100 + 1;
    }

    fn init_plan(&mut self) {
        if self.plan.is_none() {
            let plan = AttackPlan::new_randomized(&self.army, &self.map, &mut self.rng);
            let stats = execute_attack_plan(
                &self.map,
                &plan.executor_actions(&self.army),
                ATTACK_PLAN_EXECUTIONS_COUNT,
                ATTACK_PLAN_EXECUTOR_TPS,
            );

            self.plan = Some((plan, stats));
        }
    }
}
