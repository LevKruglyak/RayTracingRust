use crate::ray::Ray;
use cgmath::Vector3;

#[derive(Debug)]
pub struct Camera {
    pub focal_length: f32,
    pub origin: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
    pub max_ray_depth: u8,
}

impl Camera {
    pub fn new(
        viewport_width: f32,
        viewport_height: f32,
        focal_length: f32,
        max_ray_depth: u8,
    ) -> Self {
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);

        Self {
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vector3::new(0.0, 0.0, focal_length),
            max_ray_depth,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
            self.max_ray_depth,
        )
    }
}

#[derive(Debug)]
pub struct Scene {
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub camera: Camera,
    pub samples_per_pixel: u16,
}

#[derive(Debug)]
pub struct SceneSettings {
    pub width: f32,
    pub height: f32,
    pub viewport_ratio: f32,
    pub focal_length: f32,
    pub samples_per_pixel: u16,
    pub max_ray_depth: u8,
}

impl SceneSettings {
    pub fn build_scene(&self) -> Scene {
        let viewport_width = self.viewport_ratio;
        let viewport_height = self.viewport_ratio * self.height / self.width;

        // Camera
        let camera = Camera::new(
            viewport_width,
            viewport_height,
            self.focal_length,
            self.max_ray_depth,
        );

        Scene {
            viewport_width,
            viewport_height,
            camera,
            samples_per_pixel: self.samples_per_pixel,
        }
    }
}
