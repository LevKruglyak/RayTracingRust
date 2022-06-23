use cgmath::InnerSpace;
use rand::thread_rng;
use rand::{distributions::Uniform, prelude::Distribution};
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;

use crate::core::scene::RenderMode;
use crate::materials::Lambertian;
use crate::utils::{color::Color, ray::Ray, types::*};

use super::traits::Material;
use super::{scene::Scene, traits::Hittable};

pub struct RenderTarget {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
    pub request_redraw: bool,
}

impl RenderTarget {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height * 4],
            request_redraw: false,
        }
    }
}

fn trace_ray(scene: &Scene, world: &dyn Hittable, ray: &Ray, depth: u8) -> Color {
    // Base condition
    if depth >= scene.settings.max_ray_depth {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(ray, 0.00001, Float::INFINITY) {
        let (attenuation, scattered) = match scene.settings.mode {
            RenderMode::Full => scene.material(hit.material).scatter(ray, &hit),
            RenderMode::Clay => Lambertian::new(Color::new(0.8, 0.8, 0.8)).scatter(ray, &hit),
            RenderMode::Normal => {
                let normal = 0.5 * (hit.normal.normalize() + Vec3::new(1.0, 1.0, 1.0));
                return Color::new(normal.x, normal.y, normal.z);
            }
            RenderMode::Random => {
                return Color::new(0.0, 0.0, 0.0);
            }
        };

        if let Some(scattered) = scattered {
            attenuation * trace_ray(scene, world, &scattered, depth + 1)
        } else {
            attenuation
        }
    } else {
        scene.background.sample(ray)
    }
}

pub fn render(target: &mut RenderTarget, scene: &Scene) {
    target.request_redraw = true;

    // Build up the scene
    let ray_origin = scene.camera.ray_origin();

    let bvh = scene.build_bvh();
    let world: &dyn Hittable = if scene.settings.enable_bvh_tree {
        &bvh
    } else {
        scene
    };

    // Computationally heavy task closure
    let calculate_pixel = |(index, pixel): (usize, &mut [u8])| {
        let x = index % target.width;
        let y = index / target.width;

        // Set up rng
        let mut rng = thread_rng();
        let range = Uniform::from(0.0..=1.0);

        let mut color = Color::new(0.0, 0.0, 0.0);

        // Run for N samples
        for _ in 0..scene.settings.samples_per_pixel {
            // UV coordinates
            let u = (x as Float + range.sample(&mut rng)) / (target.width - 1) as Float;
            let v = (y as Float + range.sample(&mut rng)) / (target.height - 1) as Float;

            // Cast ray
            let mut ray_color = trace_ray(&scene, world, &ray_origin.get_ray(u, v), 0);

            // Prevent fireflies
            ray_color.r = ray_color.r.clamp(0.0, scene.settings.clamp_indirect);
            ray_color.g = ray_color.g.clamp(0.0, scene.settings.clamp_indirect);
            ray_color.b = ray_color.b.clamp(0.0, scene.settings.clamp_indirect);

            color = color + ray_color;
        }

        // Apply gamma correction
        color = Color {
            r: (color.r / scene.settings.samples_per_pixel as Float).sqrt(),
            g: (color.g / scene.settings.samples_per_pixel as Float).sqrt(),
            b: (color.b / scene.settings.samples_per_pixel as Float).sqrt(),
        };

        // Write raw data to buffer
        pixel.copy_from_slice(&color.into_raw());
    };

    // Use parallel iterators to automatically create thread pool
    if scene.settings.enable_multithreading {
        target
            .data
            .par_chunks_exact_mut(4)
            .enumerate()
            .for_each(calculate_pixel);
    } else {
        target
            .data
            .chunks_exact_mut(4)
            .enumerate()
            .for_each(calculate_pixel);
    }
}
