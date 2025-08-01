mod attack_plan;
mod attack_plan_executor;
mod attack_plan_unit;

pub use attack_plan::AttackPlan;
pub use attack_plan_executor::AttackPlanExecutor;
use attack_plan_unit::AttackPlanUnit;
use rand::seq::index::sample_array;
use rand_pcg::Pcg64Mcg;

use crate::{
    Game,
    Map,
    UnitModelEnum,
    consts::{
        ATTACK_PLAN_EXECUTIONS_COUNT,
        ATTACK_PLAN_EXECUTOR_TPS,
        NEW_POPULATION_SIZE,
        NEW_RANDOM_PLANS,
        POPULATION_SIZE,
        RNG_INITIAL_STATE,
    },
};

pub struct AttackOptimizer {
    map: Map,
    units: Vec<UnitModelEnum>,
    rng: Pcg64Mcg,
    population: Vec<(AttackPlan, AttackPlanExecutionStats)>,
    best: Option<(AttackPlan, AttackPlanExecutionStats)>,
}

impl AttackOptimizer {
    pub fn new(map: Map, units: Vec<UnitModelEnum>) -> Self {
        Self {
            map,
            units,
            rng: Pcg64Mcg::new(RNG_INITIAL_STATE),
            population: Vec::new(),
            best: None,
        }
    }

    pub fn step(&mut self) -> &(AttackPlan, AttackPlanExecutionStats) {
        let mut new_population = Vec::new();

        while new_population.len() != NEW_RANDOM_PLANS {
            let new_plan = AttackPlan::new_randomized(&self.units, &mut self.rng);
            let new_plan_stats = self.execute_attack_plan(
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
                let new_plan_stats = self.execute_attack_plan(
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

    fn execute_attack_plan(
        &self,
        plan: &AttackPlan,
        executions_count: usize,
        tps: usize,
    ) -> AttackPlanExecutionStats {
        let mut time_elapsed = Vec::with_capacity(executions_count);
        let delta_time = 1.0 / tps as f32;

        for i in 0..executions_count {
            // maybe set enable_collision_grid to true in future (when ground units will be
            // added)
            let mut game = Game::new(
                &self.map,
                false,
                Some(Pcg64Mcg::new(RNG_INITIAL_STATE + i as u128)),
            );
            let mut attack_plan_executor = AttackPlanExecutor::new(plan.units());

            while !game.done() {
                attack_plan_executor.tick(&mut game);
                game.tick(delta_time);
            }

            time_elapsed.push(game.time_elapsed());
        }

        AttackPlanExecutionStats {
            avg_time_elapsed: time_elapsed.iter().sum::<f32>() / time_elapsed.len() as f32,
            time_elapsed,
        }
    }

    pub fn best(&self) -> Option<&(AttackPlan, AttackPlanExecutionStats)> {
        self.best.as_ref()
    }

    pub fn map(&self) -> &Map {
        &self.map
    }
}

#[derive(Clone, Debug)]
pub struct AttackPlanExecutionStats {
    pub time_elapsed: Vec<f32>,
    pub avg_time_elapsed: f32,
}

impl AttackPlanExecutionStats {
    pub fn min_time_elapsed(&self) -> f32 {
        self.time_elapsed.iter().cloned().reduce(f32::min).unwrap()
    }

    pub fn max_time_elapsed(&self) -> f32 {
        self.time_elapsed.iter().cloned().reduce(f32::max).unwrap()
    }

    /// Returns Vec<(rounded time_elapsed, count)>
    pub fn merge_time_elapsed(&self) -> Vec<(usize, usize)> {
        let min_time_elapsed = self.min_time_elapsed().round() as usize;
        let max_time_elapsed = self.max_time_elapsed().round() as usize;

        let mut result = (min_time_elapsed..=max_time_elapsed)
            .map(|time_elapsed| (time_elapsed, 0usize))
            .collect::<Vec<_>>();

        for time_elapsed in &self.time_elapsed {
            result[time_elapsed.round() as usize - min_time_elapsed].1 += 1;
        }

        result
    }
}
