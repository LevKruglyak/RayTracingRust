use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracing_rust::render::RayTracingDemo;

fn criterion_benchmark(c: &mut Criterion) {
    let mut app = RayTracingDemo::new(
        100,
        100,
    );
    app.setup();
    c.bench_function("render", |b| b.iter(|| app.update()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
