mod consts;
mod utils;

use cocsim::{
    AttackOptimizer,
    AttackPlanExecutor,
    BalloonModel,
    DragonModel,
    Game,
    GeneticAttackOptimizer,
    HasteSpellModel,
    HealingSpellModel,
    LightningSpellModel,
    RageSpellModel,
    SimulatedAnnealingAttackOptimizer,
    SpellModelEnum,
    UnitModelEnum,
    WithCount,
    consts::RNG_INITIAL_STATE,
    utils::load_test_map,
};
use rand_pcg::Pcg64Mcg;

use crate::utils::macroquad_run_game;

fn main() {
    let units: Vec<WithCount<UnitModelEnum>> = vec![
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
    ];
    let spells: Vec<WithCount<SpellModelEnum>> = vec![
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
    ];

    let (map, map_image) = load_test_map("single_player/no_flight_zone").unwrap();

    let mut optimizer =
        GeneticAttackOptimizer::new(map.clone(), units.clone(), spells.clone(), 0.02, 0.02);

    for i in 0..20 {
        let (_, best_plan_stats) = optimizer.step();

        println!(
            "Gen. #{i} best plan finished in {:.1} <= {:.1} <= {:.1} seconds (avg. percentage destroyed = {})",
            best_plan_stats.min_time_elapsed,
            best_plan_stats.avg_time_elapsed,
            best_plan_stats.max_time_elapsed,
            best_plan_stats.avg_percentage_destroyed
        );
    }

    println!("Switching to SimulatedAnnealingAttackOptimizer");

    let mut optimizer = SimulatedAnnealingAttackOptimizer::new(
        map.clone(),
        units.clone(),
        spells.clone(),
        Some(optimizer.best().unwrap().clone()),
        100 * 20,
        100,
    );

    for i in 0..20 {
        let (_, best_plan_stats) = optimizer.step();

        println!(
            "Gen. #{i} best plan finished in {:.1} <= {:.1} <= {:.1} seconds (avg. percentage destroyed = {})",
            best_plan_stats.min_time_elapsed,
            best_plan_stats.avg_time_elapsed,
            best_plan_stats.max_time_elapsed,
            best_plan_stats.avg_percentage_destroyed
        );
    }

    let game = Game::new(&map, true, Some(Pcg64Mcg::new(RNG_INITIAL_STATE)));
    let mut plan_executor = AttackPlanExecutor::new(&optimizer.best().unwrap().0, &map);

    macroquad_run_game(
        game,
        map_image,
        Some(Box::new(move |game| plan_executor.tick(game))),
    );
}
