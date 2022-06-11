use crate::{
    material::{Dielectric, Emission, Lambertian, Material, Metal},
    objects::{HittableList, Sphere},
    ray::{Hittable, Ray},
    scene::SceneSettings,
};
use cgmath::Vector3;
use palette::{LinSrgba, Pixel};
use rand::Rng;
use rand::{distributions::Uniform, prelude::Distribution};
use std::{
    rc::Rc,
    time::{Duration, Instant},
};

pub struct RayTracingDemo {
    width: u32,
    height: u32,
    pixels: Vec<LinSrgba>,
    pub objects: HittableList,
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
                max_ray_depth: 6,
            },
            objects: HittableList::new(),
            last_time: Duration::new(0, 0),
            needs_redraw: true,
        }
    }

    pub fn setup(&mut self) {
        let mut rng = rand::thread_rng();
        let mat_ground = Rc::new(Lambertian::new(LinSrgba::new(0.2, 0.2, 0.2, 1.0)));

        for x in -5..5 {
            for y in -5..5 {
                let color = LinSrgba::new(rng.gen_range(0.1..1.0), rng.gen_range(0.1..1.0), rng.gen_range(0.1..1.0), 1.0);
                let material: Rc<dyn Material> = if rng.gen_bool(0.3) {
                    Rc::new(Lambertian::new(color))
                } else if rng.gen_bool(0.5) {
                    Rc::new(Metal::new(color, rng.gen_range(0.0..0.2)))
                } else if rng.gen_bool(0.6) {
                    Rc::new(Dielectric::new(1.5))
                } else {
                    Rc::new(Emission::new(color, 10.0))
                };

                let radius = rng.gen_range(0.01..0.1);

                self.objects.add(Box::new(Sphere::new(
                    Vector3::new(0.2 * (x as f32), 0.5 - radius, -1.0 - 0.2 * (y as f32)),
                    radius,
                    material,
                )));
            }
        }

        self.objects.add(Box::new(Sphere::new(
            Vector3::new(0.0, 0.5 - 1.0, -2.5),
            1.0,
            Rc::new(Metal::new(LinSrgba::new(0.7, 0.7, 0.7, 1.0), 0.02)),
        )));

        self.objects.add(Box::new(Sphere::new(
            Vector3::new(0.0, 100.5, -1.0),
            100.0,
            mat_ground,
        )));
    }

    pub fn ray_color(&mut self, ray: &Ray) -> LinSrgba {
        // Base condition
        if ray.depth <= 0 {
            return LinSrgba::new(0.0, 0.0, 0.0, 1.0);
        }

        if let Some(hit) = self.objects.hit(ray, 0.01, f32::INFINITY) {
            // Normal emissive
            // let normal = 0.5 * (hit.normal.normalize() + Vector3::new(1.0, 1.0, 1.0));
            // LinSrgba::new(normal.x, normal.y, normal.z, 1.0)

            let (attenuation, scattered) = hit.material.scatter(&ray, &hit);

            if let Some(scattered) = scattered {
                attenuation * self.ray_color(&scattered)
            } else {
                attenuation
            }
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
        let mut rng = rand::thread_rng();
        let range = Uniform::from(0.0..1.0);

        for y in 0..self.height {
            for x in 0..self.width {
                let mut color = LinSrgba::new(0.0, 0.0, 0.0, 1.0);

                // Antialiasing / sampling
                for _ in 0..self.scene.samples_per_pixel {
                    // UV coordinates
                    let u = (x as f32 + range.sample(&mut rng)) / (self.width - 1) as f32;
                    let v = (y as f32 + range.sample(&mut rng)) / (self.height - 1) as f32;

                    // Cast a ray
                    let ray = scene.camera.get_ray(u, v);
                    color += self.ray_color(&ray);
                }

                // Apply gamma correction
                color.red = (color.red / (self.scene.samples_per_pixel as f32)).sqrt();
                color.blue = (color.blue / (self.scene.samples_per_pixel as f32)).sqrt();
                color.green = (color.green / (self.scene.samples_per_pixel as f32)).sqrt();

                // Gamma correction
                self.pixels[(x + y * self.width) as usize] = color;
            }
        }

        self.last_time = current.elapsed();
        self.needs_redraw = true;
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        println!("draw");
        self.needs_redraw = false;

        assert_eq!(self.pixels.len() * 4, frame.len());
        for (pixel, result) in self.pixels.iter().zip(frame.chunks_exact_mut(4)) {
            let pixel: [u8; 4] = LinSrgba::into_raw(pixel.into_format());
            result.copy_from_slice(&pixel);
        }
    }
}
