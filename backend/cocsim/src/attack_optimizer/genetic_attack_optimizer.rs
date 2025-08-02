use rand::seq::index::sample_array;
use rand_pcg::Pcg64Mcg;

use crate::{
    AttackOptimizer,
    AttackPlan,
    AttackPlanExecutionStats,
    Map,
    UnitModelEnum,
    attack_optimizer::execute_attack_plan,
    consts::{
        ATTACK_PLAN_EXECUTIONS_COUNT,
        ATTACK_PLAN_EXECUTOR_TPS,
        NEW_POPULATION_SIZE,
        NEW_RANDOM_PLANS,
        POPULATION_SIZE,
        RNG_INITIAL_STATE,
    },
};

pub struct GeneticAttackOptimizer {
    map: Map,
    units: Vec<UnitModelEnum>,
    rng: Pcg64Mcg,
    population: Vec<(AttackPlan, AttackPlanExecutionStats)>,
    best: Option<(AttackPlan, AttackPlanExecutionStats)>,
}

impl GeneticAttackOptimizer {
    pub fn new(map: Map, units: Vec<UnitModelEnum>) -> Self {
        Self {
            map,
            units,
            rng: Pcg64Mcg::new(RNG_INITIAL_STATE),
            population: Vec::new(),
            best: None,
        }
    }
}

impl AttackOptimizer for GeneticAttackOptimizer {
    fn map(&self) -> &Map {
        &self.map
    }

    fn best(&self) -> Option<&(AttackPlan, AttackPlanExecutionStats)> {
        self.best.as_ref()
    }

    fn step(&mut self) -> &(AttackPlan, AttackPlanExecutionStats) {
        let mut new_population = Vec::new();

        while new_population.len() != NEW_RANDOM_PLANS {
            let new_plan = AttackPlan::new_randomized(&self.units, &mut self.rng);
            let new_plan_stats = execute_attack_plan(
                &self.map,
                &new_plan,
                ATTACK_PLAN_EXECUTOR_TPS,
                ATTACK_PLAN_EXECUTIONS_COUNT,
            );

            new_population.push((new_plan, new_plan_stats));
        }

        if !self.population.is_empty() {
            while new_population.len() != NEW_POPULATION_SIZE {
                let [a_index, b_index] =
                    sample_array(&mut self.rng, self.population.len()).unwrap();

                let new_plan = AttackPlan::merge(
                    &self.population[a_index].0,
                    &self.population[b_index].0,
                    &mut self.rng,
                )
                .mutate(&mut self.rng);
                let new_plan_stats = execute_attack_plan(
                    &self.map,
                    &new_plan,
                    ATTACK_PLAN_EXECUTOR_TPS,
                    ATTACK_PLAN_EXECUTIONS_COUNT,
                );

                new_population.push((new_plan, new_plan_stats));
            }
        }

        new_population
            .sort_unstable_by(|a, b| a.1.avg_time_elapsed.total_cmp(&b.1.avg_time_elapsed));

        if self.best.is_none()
            || new_population[0].1.avg_time_elapsed < self.best.as_ref().unwrap().1.avg_time_elapsed
        {
            self.best = Some(new_population[0].clone());
        }

        new_population.truncate(POPULATION_SIZE);

        self.population = new_population;

        self.best.as_ref().unwrap()
    }
}
