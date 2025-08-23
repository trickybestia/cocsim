use crate::{
    ValidatedMap,
    attack_optimizer::{
        AttackPlanExecutionStats,
        attack_plan_execution_stats::execute_attack_plan_single,
        attack_plan_executor::AttackPlanExecutorAction,
    },
};

pub fn execute_attack_plan(
    map: &ValidatedMap,
    actions: &[AttackPlanExecutorAction],
    executions_count: usize,
    tps: usize,
) -> AttackPlanExecutionStats {
    let delta_time = 1.0 / tps as f32;
    let mut executions = Vec::with_capacity(executions_count);

    for i in 0..executions_count {
        executions.push(execute_attack_plan_single(map, &actions, i, delta_time));
    }

    AttackPlanExecutionStats::new(executions)
}
