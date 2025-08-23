use nalgebra::clamp;
use rand::{
    Rng,
    seq::{
        IndexedRandom,
        index::sample_array,
    },
};
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
        NEW_POPULATION_SIZE,
        NEW_RANDOM_PLANS,
        POPULATION_SIZE,
        RNG_INITIAL_STATE,
    },
};

pub struct GeneticAttackOptimizer {
    map: ValidatedMap,
    army: Army,
    rng: Pcg64Mcg,
    population: Vec<(AttackPlan, AttackPlanExecutionStats)>,
    pub mutation_temperature: f32,
    pub mutation_temperature_decay: f32,
    pub merge_probability: f64,
    pub merge_probability_decay: f64,
}

impl GeneticAttackOptimizer {
    pub fn new(
        map: ValidatedMap,
        army: Army,
        mutation_temperature_decay: f32,
        merge_probability_decay: f64,
    ) -> Self {
        Self {
            map,
            army,
            rng: Pcg64Mcg::new(RNG_INITIAL_STATE),
            population: Vec::new(),
            mutation_temperature: 1.0,
            mutation_temperature_decay,
            merge_probability: 1.0,
            merge_probability_decay,
        }
    }
}

impl AttackOptimizer for GeneticAttackOptimizer {
    fn best(&self) -> Option<&(AttackPlan, AttackPlanExecutionStats)> {
        self.population.first()
    }

    fn step(&mut self) -> &(AttackPlan, AttackPlanExecutionStats) {
        let mut new_population = Vec::new();

        while new_population.len() != NEW_RANDOM_PLANS {
            let new_plan = AttackPlan::new_randomized(&self.army, &mut self.rng);
            let new_plan_stats = execute_attack_plan(
                &self.map,
                &new_plan.executor_actions(&self.map),
                ATTACK_PLAN_EXECUTOR_TPS,
                ATTACK_PLAN_EXECUTIONS_COUNT,
            );

            new_population.push((new_plan, new_plan_stats));
        }

        if !self.population.is_empty() {
            new_population.push(self.population[0].clone());

            while new_population.len() != NEW_POPULATION_SIZE {
                let new_plan = if self.rng.random_bool(self.merge_probability) {
                    let [a_index, b_index] =
                        sample_array(&mut self.rng, self.population.len()).unwrap();

                    AttackPlan::merge(
                        &self.population[a_index].0,
                        &self.population[b_index].0,
                        &mut self.rng,
                    )
                } else {
                    self.population.choose(&mut self.rng).unwrap().0.clone()
                };

                let new_plan = new_plan.mutate(&mut self.rng, self.mutation_temperature);
                let new_plan_stats = execute_attack_plan(
                    &self.map,
                    &new_plan.executor_actions(&self.map),
                    ATTACK_PLAN_EXECUTOR_TPS,
                    ATTACK_PLAN_EXECUTIONS_COUNT,
                );

                new_population.push((new_plan, new_plan_stats));
            }
        }

        // sort by score reversed, so highest score will be at index 0
        new_population.sort_unstable_by(|a, b| b.1.score.total_cmp(&a.1.score));

        new_population.truncate(POPULATION_SIZE);

        self.population = new_population;

        self.mutation_temperature = clamp(
            self.mutation_temperature - self.mutation_temperature_decay,
            0.0,
            1.0,
        );
        self.merge_probability = clamp(
            self.merge_probability - self.merge_probability_decay,
            0.0,
            1.0,
        );

        &self.population[0]
    }
}
