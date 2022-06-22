use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracing_rust::{
    core::mesh::Mesh,
    core::render::{render, RenderTarget},
    core::scene::Scene,
    materials::Dielectric,
};

fn criterion_benchmark(c: &mut Criterion) {
    let mut scene = Scene::default();
    scene.settings.max_ray_depth = 50;
    scene.settings.samples_per_pixel = 10;

    let default_material = scene.add_material(Box::new(Dielectric::new(1.5)));
    let mut mesh = Box::new(Mesh::from_file("assets/monkey.obj", default_material));
    mesh.build_bvh();
    scene.add_object(mesh);

    let mut target = RenderTarget::new(40, 40);

    c.bench_function("render", |b| b.iter(|| render(&mut target, &scene)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
