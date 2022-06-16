use cgmath::Vector3;
use ray_tracing_rust::{
    camera::Camera,
    color::Color,
    material::{Emission, Lambertian, Metal, Dielectric},
    objects::Sphere,
    render::RayTracingDemo,
    scene::{RenderMode, RenderSettings, Scene},
    sky::UniformBackground,
};

pub fn setup(width: u32, height: u32) -> RayTracingDemo {
    let scene = Scene::new(
        RenderSettings {
            viewport_width: width as f32,
            viewport_height: height as f32,
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
            aspect_ratio: (width as f32) / (height as f32),
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
    app
}
    // let mat_metal = Box::new(Metal::new(Color::new(1.0, 1.0, 1.0), 0.02));
    // let mat_diffuse = Box::new(Lambertian::new(Color::new(1.0, 0.2, 0.02)));
    // let mat_glass = self.scene.add_material(Box::new(Dielectric::new(1.5)));
    // let mat_sphere =
    //     self.scene
    //         .add_material(Box::new(MixMaterial::new(mat_metal, mat_diffuse, 0.9)));
    // let mat_ground = self
    //     .scene
    //     .add_material(Box::new(Lambertian::new(Color::new(0.2, 0.2, 0.2))));

    // let ground_radius = 100.0;
    // let _ground = self.scene.add_object(Box::new(Sphere::new(
    //     Vector3::new(0.0, 0.0 - ground_radius, -1.0),
    //     ground_radius,
    //     mat_ground,
    // )));
    // let _glass = self.scene.add_object(Box::new(Sphere::new(
    //     Vector3::new(0.0, 0.5, -1.0),
    //     0.5,
    //     mat_glass,
    // )));
    // let _small_ball = self.scene.add_object(Box::new(Sphere::new(
    //     Vector3::new(-0.9, 0.2, -0.7),
    //     0.2,
    //     mat_sphere,
    // )));

