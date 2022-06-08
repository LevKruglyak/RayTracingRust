use crate::{
    objects::{HittableList, Sphere},
    ray::{Hittable, Ray},
    scene::SceneSettings,
};
use cgmath::{InnerSpace, Vector3};
use palette::{LinSrgba, Pixel};
use std::time::{Duration, Instant};

pub struct RayTracingDemo {
    width: u32,
    height: u32,
    pixels: Vec<LinSrgba>,
    objects: HittableList,
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
            },
            objects: HittableList::new(),
            last_time: Duration::new(0, 0),
            needs_redraw: true,
        }
    }

    pub fn setup(&mut self) {
        self.objects
            .add(Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)));
        self.objects.add(Box::new(Sphere::new(
            Vector3::new(0.0, 1000.5, -1.0),
            1000.0,
        )));
    }

    pub fn update(&mut self) {
        let current = Instant::now();

        // Build up the scene
        let scene = self.scene.build_scene();

        for y in 0..self.height {
            for x in 0..self.width {
                // UV coordinates
                let u = (x as f32) / (self.width - 1) as f32;
                let v = (y as f32) / (self.height - 1) as f32;

                // Cast a ray
                let camera = &scene.camera;
                let ray = Ray::new(
                    camera.origin,
                    camera.lower_left_corner + camera.horizontal * u + camera.vertical * v
                        - camera.origin,
                );

                self.pixels[(x + y * self.width) as usize] =
                    if let Some(hit) = self.objects.hit(ray, 0.0, f32::INFINITY) {
                        let normal = 0.5 * (hit.normal.normalize() + Vector3::new(1.0, 1.0, 1.0));
                        LinSrgba::new(normal.x, normal.y, normal.z, 1.0)
                    } else {
                        ray.vertical_grad(
                            LinSrgba::new(0.5, 0.7, 1.0, 1.0),
                            LinSrgba::new(1.0, 1.0, 1.0, 1.0),
                        )
                    }
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
