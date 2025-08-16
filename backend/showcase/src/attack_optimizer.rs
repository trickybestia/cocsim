mod consts;
mod utils;

use cocsim::{
    AttackOptimizer,
    AttackPlanExecutor,
    DragonModel,
    Game,
    SimulatedAnnealingAttackOptimizer,
    UnitWithCount,
    consts::RNG_INITIAL_STATE,
    utils::load_test_map,
};
use rand_pcg::Pcg64Mcg;

use crate::utils::macroquad_run_game;

fn main() {
    let units: Vec<UnitWithCount> = vec![
        UnitWithCount {
            unit: DragonModel {
                level: 5.try_into().unwrap(),
            }
            .into(),
            count: 6,
        },
        UnitWithCount {
            unit: DragonModel {
                level: 5.try_into().unwrap(),
            }
            .into(),
            count: 7,
        },
    ];

    let (map, map_image) = load_test_map("single_player/no_flight_zone").unwrap();

    let mut optimizer =
        SimulatedAnnealingAttackOptimizer::new(map.clone(), units.clone(), None, 1000);

    for i in 0..10 {
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
    let mut plan_executor = AttackPlanExecutor::new(optimizer.best().unwrap().0.units.clone());

    macroquad_run_game(
        game,
        map_image,
        Some(Box::new(move |game| plan_executor.tick(game))),
    );
}
