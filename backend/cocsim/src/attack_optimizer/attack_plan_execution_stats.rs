use rand_pcg::Pcg64Mcg;

use crate::{
    AttackPlan,
    AttackPlanExecutor,
    Game,
    Map,
    consts::{
        MAX_ATTACK_DURATION,
        RNG_INITIAL_STATE,
    },
};

#[derive(Clone, Debug)]
pub struct AttackPlanExecution {
    pub time_elapsed: f32,
    /// In range [0.0; 100.0]
    pub percentage_destroyed: f32,
}

#[derive(Clone, Debug)]
pub struct AttackPlanExecutionStats {
    pub executions: Vec<AttackPlanExecution>,
    pub avg_time_elapsed: f32,
    pub avg_percentage_destroyed: f32,
}

impl AttackPlanExecutionStats {
    pub fn new(executions: Vec<AttackPlanExecution>) -> Self {
        let mut avg_time_elapsed = 0.0;
        let mut avg_percentage_destroyed = 0.0;

        for execution in &executions {
            avg_time_elapsed += execution.time_elapsed;
            avg_percentage_destroyed += execution.percentage_destroyed;
        }

        avg_time_elapsed /= executions.len() as f32;
        avg_percentage_destroyed /= executions.len() as f32;

        Self {
            executions,
            avg_time_elapsed,
            avg_percentage_destroyed,
        }
    }

    pub fn min_time_elapsed(&self) -> f32 {
        self.executions
            .iter()
            .map(|e| e.time_elapsed)
            .reduce(f32::min)
            .unwrap()
    }

    pub fn max_time_elapsed(&self) -> f32 {
        self.executions
            .iter()
            .map(|e| e.time_elapsed)
            .reduce(f32::max)
            .unwrap()
    }

    pub fn avg_time_left(&self) -> f32 {
        MAX_ATTACK_DURATION - self.avg_time_elapsed
    }

    /// More is better.
    pub fn score(&self) -> f32 {
        self.avg_time_left() * 100.0 + self.avg_percentage_destroyed
    }

    /// Returns Vec<(rounded time_elapsed, count)>
    pub fn merge_time_elapsed(&self) -> Vec<(usize, usize)> {
        let min_time_elapsed = self.min_time_elapsed().round() as usize;
        let max_time_elapsed = self.max_time_elapsed().round() as usize;

        let mut result = (min_time_elapsed..=max_time_elapsed)
            .map(|time_elapsed| (time_elapsed, 0usize))
            .collect::<Vec<_>>();

        for execution in &self.executions {
            result[execution.time_elapsed.round() as usize - min_time_elapsed].1 += 1;
        }

        result
    }
}

pub fn execute_attack_plan_single(
    map: &Map,
    plan: &AttackPlan,
    i: usize,
    delta_time: f32,
) -> AttackPlanExecution {
    // maybe set enable_collision_grid to true in future (when ground units will be
    // added)
    let mut game = Game::new(
        map,
        false,
        Some(Pcg64Mcg::new(RNG_INITIAL_STATE + i as u128)),
    );
    let mut attack_plan_executor = AttackPlanExecutor::new(&plan.units);
    let mut early_loose = false;

    while !game.done() {
        if !game.is_attacker_team_present() && attack_plan_executor.is_empty() {
            early_loose = true;

            break;
        }

        attack_plan_executor.tick(&mut game);
        game.tick(delta_time);
    }

    let time_elapsed = if early_loose {
        MAX_ATTACK_DURATION
    } else {
        game.time_elapsed()
    };

    AttackPlanExecution {
        time_elapsed,
        percentage_destroyed: game.percentage_destroyed(),
    }
}
