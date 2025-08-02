mod attack_plan;
mod attack_plan_executor;
mod attack_plan_unit;
mod derivative_attack_optimizer;
mod genetic_attack_optimizer;

pub use attack_plan::AttackPlan;
pub use attack_plan_executor::AttackPlanExecutor;
pub use attack_plan_unit::AttackPlanUnit;
pub use derivative_attack_optimizer::DerivativeAttackOptimizer;
pub use genetic_attack_optimizer::GeneticAttackOptimizer;
use rand_pcg::Pcg64Mcg;

use crate::{
    Game,
    Map,
    consts::RNG_INITIAL_STATE,
};

pub trait AttackOptimizer {
    fn map(&self) -> &Map;

    fn best(&self) -> Option<&(AttackPlan, AttackPlanExecutionStats)>;

    fn step(&mut self) -> &(AttackPlan, AttackPlanExecutionStats);
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

fn execute_attack_plan(
    map: &Map,
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
            map,
            false,
            Some(Pcg64Mcg::new(RNG_INITIAL_STATE + i as u128)),
        );
        let mut attack_plan_executor = AttackPlanExecutor::new(&plan.units);

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
