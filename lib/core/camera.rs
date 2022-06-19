use crate::utils::{
    math::degrees_to_radians,
    ray::Ray,
    types::{Float, Vec3},
};
use cgmath::InnerSpace;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, new)]
pub struct RayOrigin {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl RayOrigin {
    pub fn get_ray(&self, s: Float, t: Float) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * s - self.vertical * t - self.origin,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, new)]
pub struct Camera {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vertical: Vec3,
    pub vertical_fov: Float,
    pub aspect_ratio: Float,
}

impl Camera {
    pub fn ray_origin(&self) -> RayOrigin {
        let theta = degrees_to_radians(self.vertical_fov);
        let h = Float::tan(theta / 2.0);
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
