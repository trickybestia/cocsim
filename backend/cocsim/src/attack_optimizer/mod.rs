mod attack_plan;
mod attack_plan_execution_stats;
mod attack_plan_executor;
mod attack_plan_unit_group;
mod derivative_attack_optimizer;
#[cfg(not(feature = "rayon"))]
mod execute_attack_plan_no_rayon;
#[cfg(feature = "rayon")]
mod execute_attack_plan_rayon;
mod genetic_attack_optimizer;

pub use attack_plan::AttackPlan;
pub use attack_plan_execution_stats::AttackPlanExecutionStats;
pub use attack_plan_executor::AttackPlanExecutor;
pub use attack_plan_unit_group::AttackPlanUnitGroup;
pub use derivative_attack_optimizer::DerivativeAttackOptimizer;
#[cfg(not(feature = "rayon"))]
pub use execute_attack_plan_no_rayon::execute_attack_plan;
#[cfg(feature = "rayon")]
pub use execute_attack_plan_rayon::execute_attack_plan;
pub use genetic_attack_optimizer::GeneticAttackOptimizer;

pub trait AttackOptimizer: Send {
    fn best(&self) -> Option<&(AttackPlan, AttackPlanExecutionStats)>;

    fn step(&mut self) -> &(AttackPlan, AttackPlanExecutionStats);
}
