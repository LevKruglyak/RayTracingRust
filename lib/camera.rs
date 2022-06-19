use crate::{ray::Ray, utils::math::degrees_to_radians};
use cgmath::{InnerSpace, Vector3};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, new)]
pub struct RayOrigin {
    pub origin: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
}

impl RayOrigin {
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * s - self.vertical * t - self.origin,
            0,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, new)]
pub struct Camera {
    pub lookfrom: Vector3<f32>,
    pub lookat: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub vertical_fov: f32,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn ray_origin(&self) -> RayOrigin {
        let theta = degrees_to_radians(self.vertical_fov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = self.aspect_ratio * viewport_height;

        let w = (self.lookfrom - self.lookat).normalize();
        let u = (self.vertical.cross(w)).normalize();
        let v = w.cross(u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        RayOrigin {
            origin: self.lookfrom,
            horizontal,
            vertical,
            lower_left_corner: self.lookfrom - horizontal / 2.0 + vertical / 2.0 - w,
        }
    }
}
