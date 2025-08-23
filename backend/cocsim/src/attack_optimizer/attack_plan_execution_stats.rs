use core::f32;

use rand_pcg::Pcg64Mcg;

use crate::{
    Game,
    ValidatedMap,
    attack_optimizer::{
        AttackPlanExecutor,
        attack_plan_executor::AttackPlanExecutorAction,
    },
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
    pub min_time_elapsed: f32,
    pub max_time_elapsed: f32,
    pub avg_time_elapsed: f32,
    pub avg_percentage_destroyed: f32,
    /// More is better.
    pub score: f32,
}

impl AttackPlanExecutionStats {
    pub fn new(executions: Vec<AttackPlanExecution>) -> Self {
        let mut min_time_elapsed = f32::INFINITY;
        let mut max_time_elapsed = f32::NEG_INFINITY;
        let mut avg_time_elapsed = 0.0;

        let mut avg_percentage_destroyed = 0.0;

        for execution in &executions {
            min_time_elapsed = min_time_elapsed.min(execution.time_elapsed);
            max_time_elapsed = max_time_elapsed.max(execution.time_elapsed);
            avg_time_elapsed += execution.time_elapsed;

            avg_percentage_destroyed += execution.percentage_destroyed;
        }

        avg_time_elapsed /= executions.len() as f32;
        avg_percentage_destroyed /= executions.len() as f32;

        Self {
            executions,
            score: avg_percentage_destroyed * 4.0 + (MAX_ATTACK_DURATION - avg_time_elapsed),
            min_time_elapsed,
            avg_time_elapsed,
            max_time_elapsed,
            avg_percentage_destroyed,
        }
    }
}

pub fn execute_attack_plan_single(
    map: &ValidatedMap,
    actions: &[AttackPlanExecutorAction],
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
    let mut attack_plan_executor = AttackPlanExecutor::new(actions);
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
