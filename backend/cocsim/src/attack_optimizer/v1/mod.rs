mod attack_plan;
mod attack_plan_spell_group;
mod attack_plan_unit_group;
mod genetic_attack_optimizer;
mod random_attack_optimizer;
mod simulated_annealing_attack_optimizer;

pub use attack_plan::AttackPlan;
pub use attack_plan_spell_group::AttackPlanSpellGroup;
pub use attack_plan_unit_group::AttackPlanUnitGroup;
pub use genetic_attack_optimizer::GeneticAttackOptimizer;
pub use random_attack_optimizer::RandomAttackOptimizer;
pub use simulated_annealing_attack_optimizer::SimulatedAnnealingAttackOptimizer;

use crate::attack_optimizer::AttackPlanExecutionStats;

pub trait AttackOptimizer: Send {
    fn best(&self) -> Option<&(AttackPlan, AttackPlanExecutionStats)>;

    fn step(&mut self) -> &(AttackPlan, AttackPlanExecutionStats);
}
