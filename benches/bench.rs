use std::rc::Rc;

use cgmath::Vector3;
use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracing_rust::{
    material::{Emission, Lambertian, Metal},
    objects::Sphere,
    render::RayTracingDemo,
    color::Color,
};

fn criterion_benchmark(c: &mut Criterion) {
    let mut app = RayTracingDemo::new(100, 100);
    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.4)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.8, 0.1, 0.1)));
    let mat_left = Rc::new(Metal::new(Color::new(1.0, 1.0, 1.0), 0.03));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.08));
    let mat_glow = Rc::new(Emission::new(Color::new(1.0, 1.0, 1.0), 2.0));

    app.objects.add(Box::new(Sphere::new(
        Vector3::new(10.0, -15.8, -1.0),
        10.0,
        mat_glow,
    )));
    app.objects.add(Box::new(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        mat_left,
    )));
    app.objects.add(Box::new(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        mat_right,
    )));
    app.objects.add(Box::new(Sphere::new(
        Vector3::new(0.0, 0.2, -1.0),
        0.3,
        mat_center,
    )));
    app.objects.add(Box::new(Sphere::new(
        Vector3::new(0.0, 100.5, -1.0),
        100.0,
        mat_ground,
    )));

    c.bench_function("render", |b| b.iter(|| app.update()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
