use cocsim::{
    AttackOptimizer,
    DragonModel,
    GeneticAttackOptimizer,
    UnitWithCount,
    ValidatedMap,
    utils::load_test_map,
};
use criterion::{
    BenchmarkGroup,
    Criterion,
    criterion_group,
    criterion_main,
    measurement::Measurement,
};

fn optimize_attack(map: &ValidatedMap, units: Vec<UnitWithCount>) {
    let mut optimizer = GeneticAttackOptimizer::new(map.clone(), units.clone(), 0.02, 0.05);

    optimizer.step();
}

fn bench_with_test_map<M: Measurement>(
    mut group: BenchmarkGroup<'_, M>,
    map_path: &str,
    units: &[UnitWithCount],
) {
    let (map, _) = load_test_map(map_path).unwrap();

    group.bench_with_input(map_path, &map, |b, i| {
        b.iter(|| optimize_attack(i, units.to_owned()))
    });
}

fn attack_optimizer_bench(c: &mut Criterion) {
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

    let mut group = c.benchmark_group("Attack optimizer");

    group.sample_size(30);

    bench_with_test_map(group, "single_player/no_flight_zone", &units);
}

criterion_group!(benches, attack_optimizer_bench);
criterion_main!(benches);
