use criterion::criterion_main;

mod registry_generate;

criterion_main!(
    registry_generate::benches,
);