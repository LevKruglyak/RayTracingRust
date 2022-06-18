use crate::color::Color;
use crate::material::{Dielectric, Emission, Lambertian, Material, Metal, MixMaterial};
use crate::objects::Sphere;
use crate::ray::Ray;
use crate::scene::{RenderMode, Scene};
use cgmath::{InnerSpace, Vector3};
use rand::{distributions::Uniform, prelude::Distribution};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::{Duration, Instant};

pub struct RayTracingDemo {
    width: u32,
    height: u32,
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
                    Vector3::new(0.2 * (x as f32), -0.5 + radius, -1.0 - 0.2 * (y as f32)),
                    radius,
                    material,
                )));
            }
        }

        let glossy = self
            .scene
            .add_material(Box::new(Metal::new(Color::new(0.7, 0.7, 0.7), 0.02)));

        self.scene.add_object(Box::new(Sphere::new(
            Vector3::new(0.0, -0.5 + 1.0, -2.5),
            1.0,
            glossy,
        )));
        self.scene.add_object(Box::new(Sphere::new(
            Vector3::new(0.0, -100.5, -1.0),
            100.0,
            mat_ground,
        )));

        println!("{}", serde_json::to_string(&self.scene).unwrap());
    }

    pub fn ray_color(scene: &Scene, ray: &Ray) -> Color {
        // Base condition
        if ray.depth >= scene.settings.max_ray_depth {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = scene.hit(ray, (0.01, f32::INFINITY)) {
            let (attenuation, scattered) = match scene.settings.mode {
                RenderMode::Full => scene.material(hit.material).scatter(ray, &hit),
                RenderMode::Clay => Lambertian::new(Color::new(0.8, 0.8, 0.8)).scatter(ray, &hit),
                RenderMode::Normal => {
                    let normal = 0.5 * (hit.normal.normalize() + Vector3::new(1.0, 1.0, 1.0));
                    return Color::new(normal.x, normal.y, normal.z);
                }
                RenderMode::Random => {
                    return Color::new(0.0, 0.0, 0.0);
                }
            };

            if let Some(scattered) = scattered {
                attenuation * Self::ray_color(scene, &scattered)
            } else {
                attenuation
            }
        } else {
            scene.background.sample(ray)
        }
    }

    pub fn render(&mut self, pixels: &mut Vec<u8>) {
        let current = Instant::now();

        // Build up the scene
        let ray_origin = self.scene.camera.ray_origin();
        let range = Uniform::from(0.0..1.0);

        // Closure to do computationally heavy task
        let calculate_pixel = |(index, pixel): (usize, &mut [u8])| {
            let x = index % (self.width as usize);
            let y = index / (self.width as usize);

            let mut color = Color::new(0.0, 0.0, 0.0);
            let mut rng = thread_rng();

            for _ in 0..self.scene.settings.samples_per_pixel {
                // UV coordinates
                let u = (x as f32 + range.sample(&mut rng)) / (self.width - 1) as f32;
                let v = (y as f32 + range.sample(&mut rng)) / (self.height - 1) as f32;

                // Cast a ray
                let ray = ray_origin.get_ray(u, v);
                color = color + Self::ray_color(&self.scene, &ray);
            }

            // Gamma correction
            color = Color {
                r: (color.r / self.scene.settings.samples_per_pixel as f32).sqrt(),
                g: (color.g / self.scene.settings.samples_per_pixel as f32).sqrt(),
                b: (color.b / self.scene.settings.samples_per_pixel as f32).sqrt(),
            };

            pixel.copy_from_slice(&color.into_raw()[..]);
        };

        if self.scene.settings.enable_multithreading {
            pixels
                .par_chunks_exact_mut(4)
                .enumerate()
                .for_each(calculate_pixel);
        } else {
            pixels
                .chunks_exact_mut(4)
                .enumerate()
                .for_each(calculate_pixel);
        }

        self.last_time = current.elapsed();
        self.needs_redraw = true;
    }

    // pub fn draw(&mut self, frame: &mut [u8]) {
    //     self.needs_redraw = false;

    //     assert_eq!(self.pixels.len() * 4, frame.len());
    //     for (pixel, result) in self.pixels.iter().zip(frame.chunks_exact_mut(4)) {
    //         let pixel: [u8; 4] = pixel.into_raw();
    //         result.copy_from_slice(&pixel);
    //     }
    // }
}
