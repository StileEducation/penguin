use std::hint::black_box;
use std::sync::OnceLock;

use criterion::{criterion_group, criterion_main, Criterion};
use magnus::Value;
use pprof::criterion::{Output, PProfProfiler};

static RUBY_VM_INITIALIZED: OnceLock<()> = OnceLock::new();

fn ensure_ruby_vm() {
    RUBY_VM_INITIALIZED.get_or_init(|| unsafe { rb_sys_test_helpers::setup_ruby_unguarded() });
    let ruby = magnus::Ruby::get().unwrap();
    let load_path: Value = ruby
        .eval("$LOAD_PATH.unshift File.expand_path('../../lib', __dir__)")
        .unwrap();
    black_box(load_path);

    let required: bool = magnus::eval("require 'penguin'").unwrap();
    black_box(required);

    let disabled: bool = ruby.eval("GC.disable").unwrap();
    black_box(disabled);
}

fn bench_generation(c: &mut Criterion) {
    ensure_ruby_vm();
    c.bench_function("Penguin::ObjectId.new", |b| {
        b.iter(|| {
            // This includes the cost of eval + interop + Ruby execution.
            let v: Value = magnus::eval("Penguin::ObjectId.new").unwrap();
            black_box(v);
        })
    });
}

fn bench_string_generation(c: &mut Criterion) {
    ensure_ruby_vm();
    c.bench_function("Penguin::ObjectId.new.to_s", |b| {
        b.iter(|| {
            // This includes the cost of eval + interop + Ruby execution.
            let s: String = magnus::eval("Penguin::ObjectId.new.to_s").unwrap();
            black_box(s);
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_string_generation, bench_generation
);
criterion_main!(benches);
