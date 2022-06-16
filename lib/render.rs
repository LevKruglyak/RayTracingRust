use crate::color::Color;
use crate::material::{Dielectric, Lambertian, Metal, MixMaterial};
use crate::objects::Sphere;
use crate::ray::{Hittable, Ray};
use crate::scene::Scene;
use cgmath::Vector3;
use rand::{distributions::Uniform, prelude::Distribution};
use rand::thread_rng;
use rayon::prelude::*;
use std::time::{Duration, Instant};


pub struct RayTracingDemo {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
    pub needs_redraw: bool,
    pub scene: Scene,
    pub last_time: Duration,
}

impl RayTracingDemo {
    pub fn new(scene: Scene) -> Self {
        let width = scene.settings.viewport_width;
        let height = scene.settings.viewport_height;

        let _aspect_ratio = width / height;

        Self {
            width: width as u32,
            height: height as u32,
            pixels: vec![Color::new(1.0, 1.0, 1.0); (width * height) as usize],
            scene,
            needs_redraw: true,
            last_time: Duration::new(0, 0),
        }
    }

    pub fn setup(&mut self) {
        let mat_metal = Box::new(Metal::new(Color::new(1.0, 1.0, 1.0), 0.02));
        let mat_diffuse = Box::new(Lambertian::new(Color::new(1.0, 0.2, 0.02)));
        let mat_glass = self.scene.add_material(Box::new(Dielectric::new(1.5)));
        let mat_sphere =
            self.scene
                .add_material(Box::new(MixMaterial::new(mat_metal, mat_diffuse, 0.9)));
        let mat_ground = self
            .scene
            .add_material(Box::new(Lambertian::new(Color::new(0.2, 0.2, 0.2))));

        let ground_radius = 100.0;
        let _ground = self.scene.add_object(Box::new(Sphere::new(
            Vector3::new(0.0, 0.0 - ground_radius, -1.0),
            ground_radius,
            mat_ground,
        )));
        let _glass = self.scene.add_object(Box::new(Sphere::new(
            Vector3::new(0.0, 0.5, -1.0),
            0.5,
            mat_glass,
        )));
        let _small_ball = self.scene.add_object(Box::new(Sphere::new(
            Vector3::new(-0.9, 0.2, -0.7),
            0.2,
            mat_sphere,
        )));
    }

    pub fn ray_color(scene: &Scene, ray: &Ray) -> Color {
        // Base condition
        if ray.depth >= scene.settings.max_ray_depth {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = scene.hit(ray, (0.01, f32::INFINITY)) {
            // if self.render_normals {
            //     // Normal emissive
            //     let normal = 0.5 * (hit.normal.normalize() + Vector3::new(1.0, 1.0, 1.0));
            //     return Color::new(normal.x, normal.y, normal.z);
            // }

            let (attenuation, scattered) = scene.material(hit.material).scatter(ray, &hit);

            if let Some(scattered) = scattered {
                attenuation * Self::ray_color(scene, &scattered)
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
                let u = (x as f32 + range.sample(&mut rng)) / (self.width - 1) as f32;
                let v = (y as f32 + range.sample(&mut rng)) / (self.height - 1) as f32;

                // Cast a ray
                let ray = ray_origin.get_ray(u, v);
                *pixel = *pixel + Self::ray_color(&self.scene, &ray);
            }

            // gamma correction
            *pixel =
                Color::from(pixel.data().map(|channel| {
                    (channel / self.scene.settings.samples_per_pixel as f32).sqrt()
                }));
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
