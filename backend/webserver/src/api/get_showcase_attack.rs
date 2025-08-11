use axum::Json;
use cocsim::{
    BalloonModel,
    Game,
    utils::load_test_map,
};
use nalgebra::Vector2;
use serde_json::{
    Value,
    to_value,
};
use tokio::task::spawn_blocking;

use crate::{
    consts::{
        FPS,
        SHOWCASE_MAP,
    },
    dto_game_renderer::DtoGameRenderer,
    utils::round_floats,
};

fn get_showcase_attack_internal() -> Json<Value> {
    let (map, _) = load_test_map(SHOWCASE_MAP).expect("Map should be loaded successfully");

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

    let mut renderer = DtoGameRenderer::new(1);

    renderer.draw(&mut game);

    while !game.done() && game.is_attacker_team_present() {
        game.tick(1.0 / FPS as f32);
        renderer.draw(&mut game);
    }

    let mut result: Value = to_value(renderer.finish(&mut game)).expect("Should not fail");

    round_floats(&mut result, 2);

    result.into()
}

pub async fn get_showcase_attack() -> Json<Value> {
    spawn_blocking(get_showcase_attack_internal)
        .await
        .expect("Should not panic")
}
