mod consts;
mod utils;

use cocsim::{
    Game,
    WithCount,
    attack_optimizer::{
        Army,
        AttackPlanExecutor,
        execute_attack_plan,
        v3::AttackPlan,
    },
    consts::{
        ATTACK_PLAN_EXECUTIONS_COUNT,
        ATTACK_PLAN_EXECUTOR_TPS,
        RNG_INITIAL_STATE,
    },
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
use fastrand::Rng;
use gomez::{
    OptimizerDriver,
    algo::Lipo,
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

    let attack_plan = AttackPlan {
        map: map.clone(),
        army: army.clone(),
    };

    let mut optimizer = OptimizerDriver::builder(&attack_plan)
        .with_algo(|f, domain| Lipo::new(f, domain, Rng::new()))
        .build();

    let mut x = optimizer.next().unwrap();

    println!("0 {} {:?}", x.1, x.0);

    for i in 0..200 {
        x = optimizer.next().unwrap();

        println!("{i} {} {:?}", x.1, x.0);
    }

    let best_plan_actions = attack_plan.executor_actions(x.0.iter().cloned());

    let score = execute_attack_plan(
        &map,
        &best_plan_actions,
        ATTACK_PLAN_EXECUTIONS_COUNT,
        ATTACK_PLAN_EXECUTOR_TPS,
    );

    println!(
        "Best plan finished in {:.1} <= {:.1} <= {:.1} seconds (avg. percentage destroyed = {})",
        score.min_time_elapsed,
        score.avg_time_elapsed,
        score.max_time_elapsed,
        score.avg_percentage_destroyed
    );

    let game = Game::new(&map, true, Some(Pcg64Mcg::new(RNG_INITIAL_STATE)));
    let mut plan_executor = AttackPlanExecutor::new(best_plan_actions);

    macroquad_run_game(
        game,
        map_image,
        Some(Box::new(move |game| plan_executor.tick(game))),
    );
}
