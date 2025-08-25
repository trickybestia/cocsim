mod consts;
mod utils;

use cocsim::{
    Game,
    WithCount,
    attack_optimizer::{
        Army,
        AttackPlanExecutor,
        v2::{
            RandomAttackOptimizer,
            SimulatedAnnealingAttackOptimizer,
        },
    },
    consts::RNG_INITIAL_STATE,
    spells::{
        HasteSpellModel,
        HealingSpellModel,
        LightningSpellModel,
        RageSpellModel,
    },
    test_maps::load_test_map,
    units::{
        BalloonModel,
        DragonModel,
    },
};
use rand_pcg::Pcg64Mcg;

use crate::utils::macroquad_run_game;

fn main() {
    let army = Army {
        units: vec![
            WithCount {
                value: DragonModel {
                    level: 5.try_into().unwrap(),
                }
                .into(),
                count: 6,
            },
            WithCount {
                value: DragonModel {
                    level: 5.try_into().unwrap(),
                }
                .into(),
                count: 5,
            },
            WithCount {
                value: BalloonModel {
                    level: 6.try_into().unwrap(),
                }
                .into(),
                count: 4,
            },
            WithCount {
                value: BalloonModel {
                    level: 6.try_into().unwrap(),
                }
                .into(),
                count: 4,
            },
        ],
        spells: vec![
            WithCount {
                value: LightningSpellModel {
                    level: 7.try_into().unwrap(),
                }
                .into(),
                count: 3,
            },
            WithCount {
                value: LightningSpellModel {
                    level: 7.try_into().unwrap(),
                }
                .into(),
                count: 3,
            },
            WithCount {
                value: RageSpellModel {
                    level: 4.try_into().unwrap(),
                }
                .into(),
                count: 1,
            },
            WithCount {
                value: HealingSpellModel {
                    level: 6.try_into().unwrap(),
                }
                .into(),
                count: 1,
            },
            WithCount {
                value: HasteSpellModel {
                    level: 4.try_into().unwrap(),
                }
                .into(),
                count: 1,
            },
        ],
    };

    let (map, map_image) = load_test_map("Single Player/No Flight Zone").unwrap();

    let mut optimizer = RandomAttackOptimizer::new(map.clone(), army.clone(), 100);

    for i in 0..20 {
        optimizer.step();

        let (_, best_plan_stats) = optimizer
            .plans()
            .iter()
            .max_by(|a, b| a.1.score.total_cmp(&b.1.score))
            .unwrap();

        println!(
            "Gen. #{i} best plan finished in {:.1} <= {:.1} <= {:.1} seconds (avg. percentage destroyed = {})",
            best_plan_stats.min_time_elapsed,
            best_plan_stats.avg_time_elapsed,
            best_plan_stats.max_time_elapsed,
            best_plan_stats.avg_percentage_destroyed
        );
    }

    let best_plan = optimizer
        .plans()
        .iter()
        .max_by(|a, b| a.1.score.total_cmp(&b.1.score))
        .unwrap();

    let mut optimizer = SimulatedAnnealingAttackOptimizer::new(
        map.clone(),
        army.clone(),
        Some((best_plan.0.clone(), best_plan.1.clone())),
    );

    for i in 0..100 {
        println!("{}", optimizer.iterations_since_last_new_found());

        let (_, best_plan_stats) = optimizer.step(100);

        println!(
            "Gen. #{i} best plan finished in {:.1} <= {:.1} <= {:.1} seconds (avg. percentage destroyed = {})",
            best_plan_stats.min_time_elapsed,
            best_plan_stats.avg_time_elapsed,
            best_plan_stats.max_time_elapsed,
            best_plan_stats.avg_percentage_destroyed
        );
    }

    let best_plan = &optimizer.best().unwrap().0;

    let game = Game::new(&map, true, Some(Pcg64Mcg::new(RNG_INITIAL_STATE)));
    let mut plan_executor = AttackPlanExecutor::new(best_plan.executor_actions(&army));

    macroquad_run_game(
        game,
        map_image,
        Some(Box::new(move |game| plan_executor.tick(game))),
    );
}
