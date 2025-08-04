mod attack_plan;
mod attack_plan_execution_stats;
mod attack_plan_executor;
mod attack_plan_unit;
mod derivative_attack_optimizer;
mod genetic_attack_optimizer;

pub use attack_plan::AttackPlan;
pub use attack_plan_execution_stats::{
    AttackPlanExecutionStats,
    execute_attack_plan,
};
pub use attack_plan_executor::AttackPlanExecutor;
pub use attack_plan_unit::AttackPlanUnit;
pub use derivative_attack_optimizer::DerivativeAttackOptimizer;
pub use genetic_attack_optimizer::GeneticAttackOptimizer;

pub trait AttackOptimizer: Send {
    fn best(&self) -> Option<&(AttackPlan, AttackPlanExecutionStats)>;

    fn step(&mut self) -> &(AttackPlan, AttackPlanExecutionStats);
}
