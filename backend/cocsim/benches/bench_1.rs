#![allow(unused)]

use cocsim::{
    BalloonModel,
    Game,
    Map,
    utils::load_test_map,
};
use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use nalgebra::Vector2;

fn run(map: &Map) {
    let mut game = Game::new(map, true, None);

    for i in 0..10 {
        game.spawn_unit(
            &BalloonModel {
                level: 10.try_into().unwrap(),
            }
            .into(),
            Vector2::new(0.5, i as f32 + 0.5),
        );
    }

    while !game.done() {
        game.tick(1.0 / 60.0);
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let (map, map_image) = load_test_map("single_player/goblin_gauntlet").unwrap();

    map.validate().unwrap();

    c.bench_function("bench_1", |b| b.iter(|| run(&map)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
