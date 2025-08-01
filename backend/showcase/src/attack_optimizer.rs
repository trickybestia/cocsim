mod consts;
mod utils;

use cocsim::{
    AttackOptimizer,
    AttackPlanExecutor,
    DragonModel,
    Game,
    UnitModelEnum,
    consts::RNG_INITIAL_STATE,
    utils::load_test_map,
    validate_units,
};
use rand_pcg::Pcg64Mcg;
use textplots::Plot;

use crate::utils::macroquad_run_game;

#[macroquad::main("cocsim")]
async fn main() {
    let units: Vec<UnitModelEnum> = vec![
        DragonModel {
            level: 5.try_into().unwrap(),
        }
        .into(),
        DragonModel {
            level: 5.try_into().unwrap(),
        }
        .into(),
        DragonModel {
            level: 5.try_into().unwrap(),
        }
        .into(),
    ];

    validate_units(&units).unwrap();

    let (map, map_image) = load_test_map("single_player/goblin_gauntlet").unwrap();

    map.validate().unwrap();

    let mut optimizer = AttackOptimizer::new(map, units);

    for i in 0..10 {
        let (_, best_plan_stats) = optimizer.step();

        println!(
            "{i}: best time left: {:.1} <= {:.1} <= {:.1} seconds",
            best_plan_stats.min_time_elapsed(),
            best_plan_stats.avg_time_elapsed,
            best_plan_stats.max_time_elapsed()
        );

        let plot_data = best_plan_stats
            .merge_time_elapsed()
            .iter()
            .map(|(time_elapsed, count)| {
                (
                    *time_elapsed as f32,
                    *count as f32 / best_plan_stats.time_elapsed.len() as f32,
                )
            })
            .collect::<Vec<_>>();

        textplots::Chart::new_with_y_range(
            120,
            60,
            plot_data[0].0 as f32,
            plot_data.last().unwrap().0 as f32,
            0.0,
            1.0,
        )
        .lineplot(&textplots::Shape::Continuous(Box::new(|x| {
            plot_data[x.round() as usize - plot_data[0].0 as usize].1
        })))
        .display();
    }

    let mut game = Game::new(
        optimizer.map(),
        true,
        Some(Pcg64Mcg::new(RNG_INITIAL_STATE)),
    );
    let mut plan_executor = AttackPlanExecutor::new(optimizer.best().unwrap().0.units());

    macroquad_run_game(
        &mut game,
        &map_image,
        Some(Box::new(move |game| plan_executor.tick(game))),
    )
    .await;
}
