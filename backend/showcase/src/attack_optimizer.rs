mod consts;
mod utils;

use cocsim::{
    AttackOptimizer,
    AttackPlanExecutor,
    DragonModel,
    Game,
    UnitModelEnum,
    utils::load_test_map,
};

use crate::utils::macroquad_run_game;

#[macroquad::main("cocsim")]
async fn main() {
    let units: Vec<UnitModelEnum> = vec![
        DragonModel { level: 5 }.into(),
        DragonModel { level: 5 }.into(),
        DragonModel { level: 5 }.into(),
    ];

    let (map, map_image) = load_test_map("single_player/goblin_gauntlet").unwrap();

    let mut optimizer = AttackOptimizer::new(map, units);

    for i in 0..100 {
        let (_, best_score) = optimizer.step().unwrap();

        println!("{i}: best time left: {best_score}");
    }

    let mut game = Game::new(optimizer.map(), true).unwrap();
    let mut plan_executor = AttackPlanExecutor::new(optimizer.best().unwrap().0.units());

    macroquad_run_game(
        &mut game,
        &map_image,
        Some(Box::new(move |game| plan_executor.tick(game).unwrap())),
    )
    .await;
}
