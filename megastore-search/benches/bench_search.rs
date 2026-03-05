use criterion::{criterion_group, criterion_main, Criterion};
use megastore_search::engine::Engine;

fn bench_end_to_end(c: &mut Criterion) {
    c.bench_function("build_index_and_search_10k", |b| {
        b.iter(|| {
            let report = Engine::bench(10_000, "smartphone camera", 10).unwrap();
            report
        })
    });
}

criterion_group!(benches, bench_end_to_end);
criterion_main!(benches);
