//! Benchmark tests for weaver semconv crate
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use criterion::{criterion_group, Criterion};

use weaver_semconv::registry::SemConvRegistry;

//// Benchmark generation test
fn bench_registry_generate(c: &mut Criterion) {
    let yaml_contents = load_yaml_contents();

    let mut group = c.benchmark_group("bench_registry_generate");
    let _ = group.bench_function("bench_registry_generate", |b| {
        b.iter(|| {
            let mut registry = SemConvRegistry::default();
            for (content, provenance) in &yaml_contents {
                let _result = registry
                    .add_semconv_spec_from_string(&provenance, &content)
                    .into_result_failing_non_fatal();
            }
        });
    });
}

fn load_yaml_contents() -> Vec<(String, String)> {
    let yaml_files = vec![
        "crates/weaver_semconv/data/client.yaml",
        "crates/weaver_semconv/data/cloud.yaml",
        "crates/weaver_semconv/data/cloudevents.yaml",
        "crates/weaver_semconv/data/database.yaml",
        "crates/weaver_semconv/data/database-metrics.yaml",
        "crates/weaver_semconv/data/event.yaml",
        "crates/weaver_semconv/data/exception.yaml",
        "crates/weaver_semconv/data/faas.yaml",
        "crates/weaver_semconv/data/faas-common.yaml",
        "crates/weaver_semconv/data/faas-metrics.yaml",
        "crates/weaver_semconv/data/http.yaml",
        "crates/weaver_semconv/data/http-common.yaml",
        "crates/weaver_semconv/data/http-metrics.yaml",
        "crates/weaver_semconv/data/jvm-metrics.yaml",
        "crates/weaver_semconv/data/media.yaml",
        "crates/weaver_semconv/data/messaging.yaml",
        "crates/weaver_semconv/data/network.yaml",
        "crates/weaver_semconv/data/rpc.yaml",
        "crates/weaver_semconv/data/rpc-metrics.yaml",
        "crates/weaver_semconv/data/server.yaml",
        "crates/weaver_semconv/data/source.yaml",
        "crates/weaver_semconv/data/trace-exception.yaml",
        "crates/weaver_semconv/data/url.yaml",
        "crates/weaver_semconv/data/user-agent.yaml",
        "crates/weaver_semconv/data/vm-metrics-experimental.yaml",
        "crates/weaver_semconv/data/tls.yaml",
    ];
    // Read files into a vector of strings
    yaml_files
        .iter()
        .map(|yaml| {
            let content = read_file_to_string(yaml).expect("Failed to read file");
            let provenance = Path::new(yaml).display().to_string();
            (content, provenance)
        })
        .collect()
}

fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    _ = file.read_to_string(&mut content)?;
    Ok(content)
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_registry_generate
);

pub fn iai_benchmark_generate() -> () {
    let yaml_contents = load_yaml_contents();
    let mut registry = SemConvRegistry::default();
    for (content, provenance) in &yaml_contents {
        let _result = registry
            .add_semconv_spec_from_string(&provenance, &content)
            .into_result_failing_non_fatal();
    }
}
