use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracing_rust::{
    core::render::{render, RenderTarget},
    core::scene::Scene,
    materials::Dielectric,
    objects::Sphere,
    utils::types::{Float, Vec3},
};

fn criterion_benchmark(c: &mut Criterion) {
    let mut scene = Scene::default();
    scene.camera.lookfrom.y = -20.0;
    scene.settings.max_ray_depth = 50;

    let default_material = scene.add_material(Box::new(Dielectric::new(1.5)));

    // Add a bunch of objects
    for x in -10..10 {
        for y in -10..10 {
            for z in -10..10 {
                scene.add_object(Box::new(Sphere::new(
                    Vec3::new(x as Float, y as Float, z as Float),
                    0.5,
                    default_material,
                )));
            }
        }
    }

    c.bench_function("render", |b| b.iter(|| scene.build_bvh()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
