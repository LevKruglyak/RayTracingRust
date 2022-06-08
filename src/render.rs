use crate::{
    objects::{HittableList, Sphere},
    ray::{Hittable, Ray},
    scene::SceneSettings, utils::random_on_unit_sphere,
};
use cgmath::{InnerSpace, Vector3};
use palette::{LinSrgba, Pixel};
use rand::{
    distributions::Uniform,
    prelude::{Distribution, ThreadRng},
    Rng,
};
use std::time::{Duration, Instant};
use crate::utils::random_in_unit_sphere;

pub struct RayTracingDemo {
    width: u32,
    height: u32,
    pixels: Vec<LinSrgba>,
    objects: HittableList,
    rng: ThreadRng,
    pub scene: SceneSettings,
    pub last_time: Duration,
    pub needs_redraw: bool,
}

impl RayTracingDemo {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![LinSrgba::new(1.0, 1.0, 1.0, 1.0); (width * height) as usize],
            scene: SceneSettings {
                width: width as f32,
                height: height as f32,
                viewport_ratio: 2.0,
                focal_length: 1.0,
                samples_per_pixel: 5,
                max_ray_depth: 20,
            },
            rng: rand::thread_rng(),
            objects: HittableList::new(),
            last_time: Duration::new(0, 0),
            needs_redraw: true,
        }
    }

    pub fn setup(&mut self) {
        self.objects
            .add(Box::new(Sphere::new(Vector3::new(0.0, -0.2, -1.0), 0.5)));
        self.objects
            .add(Box::new(Sphere::new(Vector3::new(0.0, 500.3, -1.0), 500.0)));
    }

    pub fn ray_color(&mut self, ray: Ray) -> LinSrgba {
        // Base condition
        if ray.depth <= 0 {
            return LinSrgba::new(0.0, 0.0, 0.0, 1.0);
        }

        if let Some(hit) = self.objects.hit(ray, 0.01, f32::INFINITY) {
            // Diffuse material
            let target = hit.point + hit.normal + random_on_unit_sphere(&mut self.rng);
            return self.ray_color(Ray::new(hit.point, target - hit.point, ray.depth-1)) * 0.5;

            // Normal emissive
            // let normal = 0.5 * (hit.normal.normalize() + Vector3::new(1.0, 1.0, 1.0));
            // LinSrgba::new(normal.x, normal.y, normal.z, 1.0)
        } else {
            ray.vertical_grad(
                LinSrgba::new(0.5, 0.7, 1.0, 1.0),
                LinSrgba::new(1.0, 1.0, 1.0, 1.0),
            )
        }
    }

    pub fn update(&mut self) {
        let current = Instant::now();

        // Build up the scene
        let scene = self.scene.build_scene();
        let range = Uniform::from(0.0..1.0);

        for y in 0..self.height {
            for x in 0..self.width {
                let mut color = LinSrgba::new(0.0, 0.0, 0.0, 1.0);

                // Antialiasing / sampling
                for _ in 0..self.scene.samples_per_pixel {
                    // UV coordinates
                    let u = (x as f32 + range.sample(&mut self.rng)) / (self.width - 1) as f32;
                    let v = (y as f32 + range.sample(&mut self.rng)) / (self.height - 1) as f32;

                    // Cast a ray
                    let ray = scene.camera.get_ray(u, v);
                    color += self.ray_color(ray);
                }

                // Apply gamma correction
                color.red = (color.red / (self.scene.samples_per_pixel as f32)).sqrt();
                color.blue = (color.blue/ (self.scene.samples_per_pixel as f32)).sqrt();
                color.green = (color.green / (self.scene.samples_per_pixel as f32)).sqrt();

                // Gamma correction
                self.pixels[(x + y * self.width) as usize] = color;
            }
        }

        self.last_time = current.elapsed();
        self.needs_redraw = true;
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        self.needs_redraw = false;

        assert_eq!(self.pixels.len() * 4, frame.len());
        for (pixel, result) in self.pixels.iter().zip(frame.chunks_exact_mut(4)) {
            let pixel: [u8; 4] = LinSrgba::into_raw(pixel.into_format());
            result.copy_from_slice(&pixel);
        }
    }
}
