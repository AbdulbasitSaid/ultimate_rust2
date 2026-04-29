use std::hint::black_box;

use criterion::{ criterion_group, criterion_main, Criterion};
pub fn bench_marking(c: &mut Criterion) {
    use ::testing::*;

    c.bench_function("bench marking slop", |b| {
        b.iter(|| {
            sploosh(
                splish(black_box(-1), black_box(0)),
                splish(black_box(1), black_box(1)),
                splish(black_box(3), black_box(2)),
            )
        })
    });
}

criterion_group!(benches, bench_marking);
criterion_main!(benches);
