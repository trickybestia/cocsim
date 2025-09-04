mod army;
mod attack_plan_execution_stats;
mod attack_plan_executor;
#[cfg(not(feature = "rayon"))]
mod execute_attack_plan_no_rayon;
#[cfg(feature = "rayon")]
mod execute_attack_plan_rayon;
pub mod v1;
pub mod v2;
pub mod v3;

pub use army::Army;
pub use attack_plan_execution_stats::AttackPlanExecutionStats;
pub use attack_plan_executor::AttackPlanExecutor;
#[cfg(not(feature = "rayon"))]
pub use execute_attack_plan_no_rayon::execute_attack_plan;
#[cfg(feature = "rayon")]
pub use execute_attack_plan_rayon::execute_attack_plan;
