use crate::color::Color;
use crate::material::{Dielectric, Emission, Lambertian, Material, Metal};
use crate::objects::Sphere;
use crate::scene::{RenderMode, Scene};
use crate::utils::{ray::Ray, types::*};
use rand::{distributions::Uniform, prelude::Distribution};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::{Duration, Instant};

pub struct RayTracingDemo {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
    pub needs_redraw: bool,
    pub continuous_mode: bool,
    pub scene: Scene,
    pub last_time: Duration,
}

impl RayTracingDemo {
    pub fn load(width: u32, height: u32, path: &str) -> Self {
        let file_contents =
            std::fs::read_to_string(path).expect(&format!("failed to read file: {:?}", path)[..]);
        RayTracingDemo::new(width, height, serde_json::from_str(&file_contents).unwrap())
    }

    pub fn new(width: u32, height: u32, scene: Scene) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(1.0, 1.0, 1.0); (width * height) as usize],
            scene,
            needs_redraw: true,
            continuous_mode: false,
            last_time: Duration::new(0, 0),
        }
    }

    pub fn setup(&mut self) {
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
        //     Vec3::new(0.0, 0.0 - ground_radius, -1.0),
        //     ground_radius,
        //     mat_ground,
        // )));
        // let _glass = self.scene.add_object(Box::new(Sphere::new(
        //     Vec3::new(0.0, 0.5, -1.0),
        //     0.5,
        //     mat_glass,
        // )));
        // let _small_ball = self.scene.add_object(Box::new(Sphere::new(
        //     Vec3::new(-0.9, 0.2, -0.7),
        //     0.2,
        //     mat_sphere,
        // )));
        let mut rng = rand::thread_rng();
        let mat_ground = self
            .scene
            .add_material(Box::new(Lambertian::new(Color::new(0.2, 0.2, 0.2))));

        for x in -5..5 {
            for y in -5..5 {
                let color = Color::new(
                    rng.gen_range(0.1..1.0),
                    rng.gen_range(0.1..1.0),
                    rng.gen_range(0.1..1.0),
                );
                let material = if rng.gen_bool(0.3) {
                    self.scene.add_material(Box::new(Lambertian::new(color)))
                } else if rng.gen_bool(0.5) {
                    self.scene
                        .add_material(Box::new(Metal::new(color, rng.gen_range(0.0..0.2))))
                } else if rng.gen_bool(0.6) {
                    self.scene.add_material(Box::new(Dielectric::new(1.5)))
                } else {
                    self.scene
                        .add_material(Box::new(Emission::new(color, 10.0)))
                };

                let radius = rng.gen_range(0.01..0.1);

                self.scene.add_object(Box::new(Sphere::new(
                    Vec3::new(0.2 * (x as Float), -0.5 + radius, -1.0 - 0.2 * (y as Float)),
                    radius,
                    material,
                )));
            }
        }

        let glossy = self
            .scene
            .add_material(Box::new(Metal::new(Color::new(0.7, 0.7, 0.7), 0.02)));

        self.scene.add_object(Box::new(Sphere::new(
            Vec3::new(0.0, -0.5 + 1.0, -2.5),
            1.0,
            glossy,
        )));
        self.scene.add_object(Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            mat_ground,
        )));

        println!("{}", serde_json::to_string(&self.scene).unwrap());
    }

    pub fn ray_color(scene: &Scene, ray: &Ray, depth: u8) -> Color {
        // Base condition
        if depth >= scene.settings.max_ray_depth {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = scene.hit(ray, 0.001, Float::INFINITY) {
            let (attenuation, scattered) = match scene.settings.mode {
                RenderMode::Full => scene.material(hit.material).scatter(ray, &hit),
                RenderMode::Clay => Lambertian::new(Color::new(0.8, 0.8, 0.8)).scatter(ray, &hit),
                RenderMode::Normal => {
                    // let normal = 0.5 * (hit.normal.normalize() + Vec3::new(1.0, 1.0, 1.0));
                    // return Color::new(normal.x, normal.y, normal.z);
                    return Color::new(0.0, 0.0, 0.0);
                }
                RenderMode::Random => {
                    return Color::new(0.0, 0.0, 0.0);
                }
            };

            if let Some(scattered) = scattered {
                attenuation * Self::ray_color(scene, &scattered, depth + 1)
            } else {
                attenuation
            }
        } else {
            scene.background.sample(ray)
        }
    }

    pub fn update(&mut self) {
        let current = Instant::now();

        // Build up the scene
        let ray_origin = self.scene.camera.ray_origin();
        let range = Uniform::from(0.0..1.0);

        // Closure to do computationally heavy task
        let calculate_pixel = |(index, pixel): (usize, &mut Color)| {
            let x = index % (self.width as usize);
            let y = index / (self.width as usize);

            *pixel = Color::new(0.0, 0.0, 0.0);
            let mut rng = thread_rng();

            for _ in 0..self.scene.settings.samples_per_pixel {
                // UV coordinates
                let u = (x as Float + range.sample(&mut rng)) / (self.width - 1) as Float;
                let v = (y as Float + range.sample(&mut rng)) / (self.height - 1) as Float;

                // Cast a ray
                let ray = ray_origin.get_ray(u, v);
                *pixel = *pixel + Self::ray_color(&self.scene, &ray, 0);
            }

            // Gamma correction
            *pixel = Color {
                r: (pixel.r / self.scene.settings.samples_per_pixel as Float).sqrt(),
                g: (pixel.g / self.scene.settings.samples_per_pixel as Float).sqrt(),
                b: (pixel.b / self.scene.settings.samples_per_pixel as Float).sqrt(),
            }
        };

        if self.scene.settings.enable_multithreading {
            self.pixels
                .par_iter_mut()
                .enumerate()
                .for_each(calculate_pixel);
        } else {
            self.pixels.iter_mut().enumerate().for_each(calculate_pixel);
        }

        self.last_time = current.elapsed();
        self.needs_redraw = true;
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        self.needs_redraw = false;

        assert_eq!(self.pixels.len() * 4, frame.len());
        for (pixel, result) in self.pixels.iter().zip(frame.chunks_exact_mut(4)) {
            let pixel: [u8; 4] = pixel.into_raw();
            result.copy_from_slice(&pixel);
        }
    }
}
