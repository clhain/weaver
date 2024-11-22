//! Benchmark tests for weaver semconv crate
use criterion::criterion_main;

mod registry_generate;

criterion_main!(
    registry_generate::benches,
);