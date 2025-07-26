mod consts;
mod utils;

use cocsim::{
    DragonModel,
    Game,
    utils::load_test_map,
};
use macroquad::prelude::*;
use nalgebra::Vector2;

use crate::utils::macroquad_run_game;

#[macroquad::main("cocsim")]
async fn main() {
    let (map, map_image) = load_test_map("single_player/goblin_gauntlet").unwrap();

    let mut game = Game::new(&map, true).unwrap();

    for i in 0..10 {
        game.spawn_unit(
            &DragonModel { level: 10 }.into(),
            Vector2::new(0.5, i as f32 + 0.5),
        )
        .unwrap();
    }

    macroquad_run_game(&mut game, &map_image, None).await;
}
