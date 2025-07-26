use axum::Json;
use cocsim::{
    DragonModel,
    Game,
    utils::load_test_map,
};
use nalgebra::Vector2;
use serde_json::{
    Value,
    to_value,
};

use crate::{
    consts::{
        FPS,
        SHOWCASE_MAP,
    },
    dto_game_renderer::DtoGameRenderer,
    utils::round_floats,
};

pub async fn get_showcase_attack() -> Json<Value> {
    let (map, _) = load_test_map(SHOWCASE_MAP).expect("Map should be loaded successfully");

    let mut game = Game::new(&map).expect("Test map should be valid");

    for i in 0..10 {
        game.spawn_unit(
            &DragonModel { level: 10 }.into(),
            Vector2::new(0.5, i as f32 + 0.5),
        )
        .expect("Unit model should be valid");
    }

    let mut renderer = DtoGameRenderer::new(1);

    renderer.draw(&mut game);

    while !game.done() {
        game.tick(1.0 / FPS as f32);
        renderer.draw(&mut game);
    }

    let mut result: Value = to_value(renderer.finish(&mut game)).expect("Should not fail");

    round_floats(&mut result, 2);

    result.into()
}
