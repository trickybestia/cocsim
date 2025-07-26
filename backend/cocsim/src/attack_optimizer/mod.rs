mod attack_plan;
mod attack_plan_executor;
mod attack_plan_unit;
mod geometry;

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
        ATTACK_PLAN_EXECUTOR_FPS,
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
    population: Vec<(AttackPlan, f32)>,
    best: Option<(AttackPlan, f32)>,
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

    pub fn step(&mut self) -> anyhow::Result<&(AttackPlan, f32)> {
        let mut new_population = Vec::new();

        while new_population.len() != NEW_RANDOM_PLANS {
            let new_plan = AttackPlan::new_randomized(&self.units, &mut self.rng);
            let new_plan_score = self.score_attack_plan(&new_plan)?;

            new_population.push((new_plan, new_plan_score));
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
                let new_plan_score = self.score_attack_plan(&new_plan)?;

                new_population.push((new_plan, new_plan_score));
            }
        }

        // sort by score reversed
        new_population.sort_unstable_by(|a, b| b.1.total_cmp(&a.1));

        if self.best.is_none() || new_population[0].1 > self.best.as_ref().unwrap().1 {
            self.best = Some(new_population[0].clone());
        }

        new_population.truncate(POPULATION_SIZE);

        self.population = new_population;

        Ok(self.best.as_ref().unwrap())
    }

    fn score_attack_plan(&mut self, plan: &AttackPlan) -> anyhow::Result<f32> {
        // maybe set enable_collision_grid to true in future (when ground units will be
        // added)
        let mut game = Game::new(&self.map, false)?;
        let mut attack_plan_executor = AttackPlanExecutor::new(plan.units());

        while !game.done() {
            attack_plan_executor.tick(&mut game)?;
            game.tick(1.0 / ATTACK_PLAN_EXECUTOR_FPS as f32);
        }

        Ok(game.time_left())
    }

    pub fn best(&self) -> Option<&(AttackPlan, f32)> {
        self.best.as_ref()
    }

    pub fn map(&self) -> &Map {
        &self.map
    }
}
