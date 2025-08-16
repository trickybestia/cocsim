use cocsim::{
    AttackOptimizer,
    DragonModel,
    GeneticAttackOptimizer,
    LightningSpellModel,
    SpellWithCount,
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

fn optimize_attack(map: &ValidatedMap, units: &[UnitWithCount], spells: &[SpellWithCount]) {
    let mut optimizer =
        GeneticAttackOptimizer::new(map.clone(), units.to_owned(), spells.to_owned(), 0.02, 0.05);

    optimizer.step();
}

fn bench_with_test_map<M: Measurement>(
    mut group: BenchmarkGroup<'_, M>,
    map_path: &str,
    units: &[UnitWithCount],
    spells: &[SpellWithCount],
) {
    let (map, _) = load_test_map(map_path).unwrap();

    group.bench_with_input(map_path, &map, |b, i| {
        b.iter(|| optimize_attack(i, units, spells))
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
    let spells: Vec<SpellWithCount> = vec![SpellWithCount {
        spell: LightningSpellModel {
            level: 7.try_into().unwrap(),
        }
        .into(),
        count: 11,
    }];

    let group = c.benchmark_group("Attack optimizer");

    bench_with_test_map(group, "single_player/no_flight_zone", &units, &spells);
}

criterion_group!(benches, attack_optimizer_bench);
criterion_main!(benches);
