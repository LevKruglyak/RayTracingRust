use cgmath::{InnerSpace, Vector3};
use palette::{LinSrgba, Pixel};

use crate::{ray::Ray, scene::Scene};

pub struct RayTracingDemo {
    width: u32,
    height: u32,
    pixels: Vec<LinSrgba>,
    scene: Scene,
    pub needs_redraw: bool,
}

impl RayTracingDemo {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![LinSrgba::new(1.0, 1.0, 1.0, 1.0); (width * height) as usize],
            scene: Scene::new(width, height),
            needs_redraw: true,
        }
    }

    pub fn update(&mut self, param: f32) {
        for x in 0..self.width {
            for y in 0..self.height {
                // UV coordinates
                let u = (x as f32) / (self.width - 1) as f32;
                let v = (y as f32) / (self.height - 1) as f32;

                // Cast a ray
                let camera = &self.scene.camera;
                let r = Ray::new(
                    camera.origin,
                    camera.lower_left_corner + camera.horizontal * u + camera.vertical * v
                        - camera.origin,
                );

                self.pixels[(x + y * self.width) as usize] = r.vertical_grad(
                    LinSrgba::new(0.5, param, 1.0, 1.0),
                    LinSrgba::new(1.0, 1.0, 1.0, 1.0),
                );
            }
        }

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
