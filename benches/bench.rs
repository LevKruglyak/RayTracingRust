use cgmath::Vector3;
use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracing_rust::{
    camera::Camera,
    color::Color,
    material::{Emission, Lambertian, Metal, Dielectric},
    objects::Sphere,
    render::RayTracingDemo,
    scene::{RenderMode, RenderSettings, Scene},
    sky::UniformBackground,
};

fn criterion_benchmark(c: &mut Criterion) {
    let scene = Scene::new(
        RenderSettings {
            viewport_width: 100.0,
            viewport_height: 100.0,
            samples_per_pixel: 5,
            max_ray_depth: 6,
            enable_multithreading: true,
            mode: RenderMode::Full,
        },
        Camera {
            lookfrom: Vector3::new(0.0, 0.0, 0.0),
            lookat: Vector3::new(0.0, 0.0, -1.0),
            vertical: Vector3::new(0.0, 1.0, 0.0),
            vertical_fov: 90.0,
            aspect_ratio: 1.0,
        },
        Box::new(UniformBackground::new(Color::new(0.6, 0.6, 0.6))),
    );
    let mut app = RayTracingDemo::new(scene);
    let mat_ground = app
        .scene
        .add_material(Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.4))));
    let mat_center = app
        .scene
        .add_material(Box::new(Lambertian::new(Color::new(0.8, 0.1, 0.1))));
    let mat_left = app
        .scene
        .add_material(Box::new(Metal::new(Color::new(1.0, 1.0, 1.0), 0.03)));
    let mat_right = app
        .scene
        .add_material(Box::new(Dielectric::new(1.5)));
    let mat_glow = app
        .scene
        .add_material(Box::new(Emission::new(Color::new(1.0, 1.0, 1.0), 2.0)));

    app.scene.add_object(Box::new(Sphere::new(
        Vector3::new(10.0, 15.8, -1.0),
        10.0,
        mat_glow,
    )));
    app.scene.add_object(Box::new(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        mat_left,
    )));
    app.scene.add_object(Box::new(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        mat_right,
    )));
    app.scene.add_object(Box::new(Sphere::new(
        Vector3::new(0.0, -0.2, -1.0),
        0.3,
        mat_center,
    )));
    app.scene.add_object(Box::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        mat_ground,
    )));

    c.bench_function("render", |b| b.iter(|| app.update()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
