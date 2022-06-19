use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracing_rust::{scene::Scene, material::Dielectric, objects::Sphere, utils::types::{Float, Vec3}, render::RayTracingDemo};

fn criterion_benchmark(c: &mut Criterion) {
    let mut scene = Scene::default();
    scene.settings.enable_bvh_tree = true;

    let default_material = scene.add_material(Box::new(Dielectric::new(1.5)));

    // Add a bunch of objects
    for x in -5..5 {
        for y in -5..5 {
                scene.add_object(Box::new(Sphere::new(
                        Vec3::new(x as Float, y as Float, 1.0),
                        0.5,
                        default_material,
                        )));
        }
    }

    let mut app = RayTracingDemo::new(100, 100, scene);
    c.bench_function("render", |b| b.iter(|| app.update()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
