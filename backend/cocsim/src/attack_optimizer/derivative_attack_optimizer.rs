use nalgebra::clamp;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::{
    AttackOptimizer,
    AttackPlan,
    AttackPlanExecutionStats,
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

pub struct DerivativeAttackOptimizer {
    map: ValidatedMap,
    units: Vec<UnitWithCount>,
    rng: Pcg64Mcg,
    plan: Option<(AttackPlan, AttackPlanExecutionStats)>,
}

impl DerivativeAttackOptimizer {
    pub fn new(
        map: ValidatedMap,
        units: Vec<UnitWithCount>,
        initial_plan: Option<(AttackPlan, AttackPlanExecutionStats)>,
    ) -> Self {
        Self {
            map,
            units,
            rng: Pcg64Mcg::new(RNG_INITIAL_STATE),
            plan: initial_plan,
        }
    }

    fn init_plan(&mut self) {
        if self.plan.is_none() {
            let plan = AttackPlan::new_randomized(&self.units, &mut self.rng);
            let stats = execute_attack_plan(
                &self.map,
                &plan,
                ATTACK_PLAN_EXECUTIONS_COUNT,
                ATTACK_PLAN_EXECUTOR_TPS,
            );

            self.plan = Some((plan, stats))
        }
    }
}

impl AttackOptimizer for DerivativeAttackOptimizer {
    fn best(&self) -> Option<&(AttackPlan, AttackPlanExecutionStats)> {
        self.plan.as_ref()
    }

    fn step(&mut self) -> &(AttackPlan, AttackPlanExecutionStats) {
        if self.plan.is_none() {
            self.init_plan();
        }

        let (plan, stats) = self.plan.as_mut().unwrap();

        for i in 0..plan.units.len() {
            let unit = &mut plan.units[i];
            let prev_drop_time = unit.drop_time;
            let drop_time_delta: f32 = self.rng.random_range(-1.0..=1.0);
            let new_drop_time = clamp(unit.drop_time + drop_time_delta, 0.0, MAX_UNIT_DROP_TIME);

            unit.drop_time = new_drop_time;

            let new_stats = execute_attack_plan(
                &self.map,
                plan,
                ATTACK_PLAN_EXECUTIONS_COUNT,
                ATTACK_PLAN_EXECUTOR_TPS,
            );

            let score_delta = new_stats.score() - stats.score(); // > 0 is better, < 0 is worse

            if score_delta >= 0.0 {
                *stats = new_stats;
            } else {
                plan.units[i].drop_time = prev_drop_time;
            }
        }

        for i in 0..plan.units.len() {
            let unit = &mut plan.units[i];
            let prev_distance = unit.distance;
            let distance_delta: f32 = self.rng.random_range(-0.5..=0.5);
            let new_distance = clamp(unit.distance + distance_delta, 0.0, 1.0);

            unit.distance = new_distance;

            let new_stats = execute_attack_plan(
                &self.map,
                plan,
                ATTACK_PLAN_EXECUTIONS_COUNT,
                ATTACK_PLAN_EXECUTOR_TPS,
            );

            let score_delta = new_stats.score() - stats.score(); // > 0 is better, < 0 is worse

            if score_delta >= 0.0 {
                *stats = new_stats;
            } else {
                plan.units[i].distance = prev_distance;
            }
        }

        for i in 0..plan.units.len() {
            let unit = &mut plan.units[i];
            let prev_angle = unit.angle;
            let angle_delta: f32 = self.rng.random_range(-0.5..=0.5);
            unit.angle = unit.angle + angle_delta;

            let new_stats = execute_attack_plan(
                &self.map,
                plan,
                ATTACK_PLAN_EXECUTIONS_COUNT,
                ATTACK_PLAN_EXECUTOR_TPS,
            );

            let score_delta = new_stats.score() - stats.score(); // > 0 is better, < 0 is worse

            if score_delta >= 0.0 {
                *stats = new_stats;
            } else {
                plan.units[i].angle = prev_angle;
            }
        }

        self.plan.as_ref().unwrap()
    }
}
