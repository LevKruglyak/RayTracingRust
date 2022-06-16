use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracing_rust::render::RayTracingDemo;

fn criterion_benchmark(c: &mut Criterion) {
    let contents =
        std::fs::read_to_string("scenes/benchmark.json").expect("error: could not read file!");
    let scene = serde_json::from_str(&contents).unwrap();
    let mut app = RayTracingDemo::new(scene);

    c.bench_function("render", |b| b.iter(|| app.update()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
