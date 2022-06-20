use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracing_rust::{
    core::render::{render, RenderTarget},
    core::scene::Scene,
};

fn criterion_benchmark(c: &mut Criterion) {
    let scene = Scene::from_file("scenes/simple.json");
    let mut target = RenderTarget::new(100, 100);

    c.bench_function("render", |b| b.iter(|| render(&mut target, &scene)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
