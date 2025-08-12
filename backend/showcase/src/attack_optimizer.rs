mod consts;
mod utils;

use cocsim::{
    AttackOptimizer,
    AttackPlanExecutor,
    BalloonModel,
    DerivativeAttackOptimizer,
    Game,
    GeneticAttackOptimizer,
    UnitWithCount,
    consts::RNG_INITIAL_STATE,
    utils::load_test_map,
};
use rand_pcg::Pcg64Mcg;
use textplots::Plot;

use crate::utils::macroquad_run_game;

fn main() {
    let units: Vec<UnitWithCount> = vec![UnitWithCount {
        unit: BalloonModel {
            level: 10.try_into().unwrap(),
        }
        .into(),
        count: 10,
    }];

    let (map, map_image) = load_test_map("single_player/no_flight_zone").unwrap();

    let mut optimizer: Box<dyn AttackOptimizer> = Box::new(GeneticAttackOptimizer::new(
        map.clone(),
        units.clone(),
        0.02,
        0.05,
    ));

    for i in 0..40 {
        if i == 20 {
            println!("Switching to DerivativeAttackOptimizer");

            optimizer = Box::new(DerivativeAttackOptimizer::new(
                map.clone(),
                units.clone(),
                optimizer.best().cloned(),
            ));
        }

        let (_, best_plan_stats) = optimizer.step();

        println!(
            "Gen. #{i} best plan finished in {:.1} <= {:.1} <= {:.1} seconds (avg. percentage destroyed = {})",
            best_plan_stats.min_time_elapsed(),
            best_plan_stats.avg_time_elapsed,
            best_plan_stats.max_time_elapsed(),
            best_plan_stats.avg_percentage_destroyed
        );

        let plot_data = best_plan_stats
            .merge_time_elapsed()
            .iter()
            .map(|(time_elapsed, count)| {
                (
                    *time_elapsed as f32,
                    *count as f32 / best_plan_stats.executions.len() as f32,
                )
            })
            .collect::<Vec<_>>();

        let mut chart = textplots::Chart::new_with_y_range(
            120,
            60,
            plot_data[0].0 as f32,
            plot_data.last().unwrap().0 as f32,
            0.0,
            1.0,
        );
        let line = textplots::Shape::Continuous(Box::new(|x| {
            plot_data[x.round() as usize - plot_data[0].0 as usize].1
        }));
        let chart = chart.lineplot(&line);

        chart.borders();
        chart.display();
    }

    let game = Game::new(&map, true, Some(Pcg64Mcg::new(RNG_INITIAL_STATE)));
    let mut plan_executor = AttackPlanExecutor::new(optimizer.best().unwrap().0.units.clone());

    macroquad_run_game(
        game,
        map_image,
        Some(Box::new(move |game| plan_executor.tick(game))),
    );
}
