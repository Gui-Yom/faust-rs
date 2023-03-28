use criterion::{criterion_group, criterion_main, Criterion};
use criterion_polyglot::{BenchSpec, CriterionPolyglotExt};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.c_benchmark(
        "test",
        BenchSpec::new(
            r#"
    int k = 13;
    for (int i = 0; i < 100000; ++i) {
        k *= 2*i;
    }
    "#,
        ),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
