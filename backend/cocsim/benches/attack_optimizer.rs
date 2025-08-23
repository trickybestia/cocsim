use cocsim::{
    DragonModel,
    LightningSpellModel,
    ValidatedMap,
    WithCount,
    attack_optimizer::{
        Army,
        v1::{
            AttackOptimizer,
            GeneticAttackOptimizer,
        },
    },
    utils::load_test_map,
};
use criterion::{
    BenchmarkGroup,
    Criterion,
    criterion_group,
    criterion_main,
    measurement::Measurement,
};

fn optimize_attack(map: &ValidatedMap, army: &Army) {
    let mut optimizer = GeneticAttackOptimizer::new(map.clone(), army.clone(), 0.02, 0.05);

    optimizer.step();
}

fn bench_with_test_map<M: Measurement>(
    mut group: BenchmarkGroup<'_, M>,
    map_path: &str,
    army: &Army,
) {
    let (map, _) = load_test_map(map_path).unwrap();

    group.bench_with_input(map_path, &map, |b, i| b.iter(|| optimize_attack(i, army)));
}

fn attack_optimizer_bench(c: &mut Criterion) {
    let army = Army {
        units: vec![
            WithCount {
                value: DragonModel {
                    level: 5.try_into().unwrap(),
                }
                .into(),
                count: 6,
            },
            WithCount {
                value: DragonModel {
                    level: 5.try_into().unwrap(),
                }
                .into(),
                count: 7,
            },
        ],
        spells: vec![WithCount {
            value: LightningSpellModel {
                level: 7.try_into().unwrap(),
            }
            .into(),
            count: 11,
        }],
    };

    let group = c.benchmark_group("Attack optimizer");

    bench_with_test_map(group, "single_player/no_flight_zone", &army);
}

criterion_group!(benches, attack_optimizer_bench);
criterion_main!(benches);
