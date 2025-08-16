use nalgebra::clamp;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::{
    AttackOptimizer,
    AttackPlan,
    AttackPlanExecutionStats,
    SpellWithCount,
    UnitWithCount,
    ValidatedMap,
    consts::{
        ATTACK_PLAN_EXECUTIONS_COUNT,
        ATTACK_PLAN_EXECUTOR_TPS,
        MAX_UNIT_DROP_TIME,
        RNG_INITIAL_STATE,
    },
    execute_attack_plan,
};

pub struct SimulatedAnnealingAttackOptimizer {
    map: ValidatedMap,
    units: Vec<UnitWithCount>,
    spells: Vec<SpellWithCount>,
    rng: Pcg64Mcg,
    plan: Option<(AttackPlan, AttackPlanExecutionStats)>,
    iterations_per_step: usize,
    iterations: usize,
    current_iteration: usize,
}

impl SimulatedAnnealingAttackOptimizer {
    pub fn new(
        map: ValidatedMap,
        units: Vec<UnitWithCount>,
        spells: Vec<SpellWithCount>,
        initial_plan: Option<(AttackPlan, AttackPlanExecutionStats)>,
        iterations: usize,
        iterations_per_step: usize,
    ) -> Self {
        Self {
            map,
            units,
            spells,
            rng: Pcg64Mcg::new(RNG_INITIAL_STATE),
            plan: initial_plan,
            iterations,
            iterations_per_step,
            current_iteration: 0,
        }
    }

    fn init_plan(&mut self) {
        if self.plan.is_none() {
            let plan = AttackPlan::new_randomized(&self.units, &self.spells, &mut self.rng);
            let stats = execute_attack_plan(
                &self.map,
                &plan,
                ATTACK_PLAN_EXECUTIONS_COUNT,
                ATTACK_PLAN_EXECUTOR_TPS,
            );

            self.plan = Some((plan, stats));
        }
    }

    fn next_value(
        value: f32,
        min: f32,
        max: f32,
        scale: f32,
        temperature: f32,
        rng: &mut impl Rng,
    ) -> f32 {
        clamp(
            value + rng.random_range(-scale..=scale) * temperature,
            min,
            max,
        )
    }
}

impl AttackOptimizer for SimulatedAnnealingAttackOptimizer {
    fn best(&self) -> Option<&(AttackPlan, AttackPlanExecutionStats)> {
        self.plan.as_ref()
    }

    fn step(&mut self) -> &(AttackPlan, AttackPlanExecutionStats) {
        if self.plan.is_none() {
            self.init_plan();
        }

        let (plan, stats) = self.plan.as_mut().unwrap();

        for _ in 0..self.iterations_per_step {
            if self.current_iteration == self.iterations {
                break;
            }

            let mut new_plan = plan.clone();

            let temperature = 1.0 - self.current_iteration as f32 / self.iterations as f32;

            for unit in &mut new_plan.units {
                unit.drop_time = Self::next_value(
                    unit.drop_time,
                    0.0,
                    MAX_UNIT_DROP_TIME,
                    10.0,
                    temperature,
                    &mut self.rng,
                );
                unit.distance =
                    Self::next_value(unit.distance, 0.0, 1.0, 5.0, temperature, &mut self.rng);
                unit.angle = Self::next_value(
                    unit.angle,
                    -10000.0,
                    10000.0,
                    5.0,
                    temperature,
                    &mut self.rng,
                );
            }

            let new_stats = execute_attack_plan(
                &self.map,
                &new_plan,
                ATTACK_PLAN_EXECUTIONS_COUNT,
                ATTACK_PLAN_EXECUTOR_TPS,
            );

            if new_stats.score >= stats.score {
                *stats = new_stats;
                *plan = new_plan;
            }

            self.current_iteration += 1;
        }

        self.plan.as_ref().unwrap()
    }
}
