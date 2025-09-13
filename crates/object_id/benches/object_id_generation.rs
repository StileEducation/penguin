use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use pprof::criterion::{Output, PProfProfiler};

fn bench_generation(c: &mut Criterion) {
    c.bench_function("generate", |b| {
        b.iter(|| black_box(object_id::ObjectId::new()))
    });
}

fn bench_string_generation(c: &mut Criterion) {
    c.bench_function("string_generation", |b| {
        b.iter(|| black_box(object_id::ObjectId::new().to_string()))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_generation, bench_string_generation
);
criterion_main!(benches);
