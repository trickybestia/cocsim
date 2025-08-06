mod consts;
mod utils;

use cocsim::{
    BalloonModel,
    Game,
    utils::load_test_map,
};
use nalgebra::Vector2;

use crate::utils::macroquad_run_game;

fn main() {
    let (map, map_image) = load_test_map("single_player/no_flight_zone").unwrap();

    map.validate().unwrap();

    let mut game = Game::new(&map, true, None);

    for _i in 0..10 {
        game.spawn_attack_unit(
            &BalloonModel {
                level: 10.try_into().unwrap(),
            }
            .into(),
            Vector2::new(0.5, 0.5),
        );
    }

    macroquad_run_game(game, map_image, None);
}
