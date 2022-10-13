#[macro_use]
extern crate criterion;
use criterion::Criterion;

use prime_factorization::Factorization;

fn bench_factorization(c: &mut Criterion) {
    let mut group = c.benchmark_group("factorization::run");
    group.sample_size(15);

    group.bench_function("u32::MAX", |b| {
        let number = u32::MAX;
        b.iter(|| Factorization::run(number))
    });

    group.bench_function("u64::MAX", |b| {
        let number = u64::MAX;
        b.iter(|| Factorization::run(number))
    });

    group.bench_function("u128::MAX", |b| {
        let number = u128::MAX;
        b.iter(|| Factorization::run(number))
    });

    group.bench_function("u128_semiprime", |b| {
        let number = 5_316_911_983_139_663_122_320_058_796_740_706_329u128;
        b.iter(|| Factorization::run(number))
    });

    group.finish();
}

criterion_group!(benches, bench_factorization);
criterion_main!(benches);
