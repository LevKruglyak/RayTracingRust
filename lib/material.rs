use cgmath::InnerSpace;
use palette::LinSrgba;

use crate::{
    ray::{HitRecord, Ray},
    utils::{near_zero, random_on_unit_sphere, reflect},
};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> (LinSrgba, Option<Ray>);
}

pub struct Lambertian {
    albedo: LinSrgba,
}

impl Lambertian {
    pub fn new(albedo: LinSrgba) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> (LinSrgba, Option<Ray>) {
        let mut scatter_direction = hit.normal + random_on_unit_sphere();

        if near_zero(scatter_direction) {
            // Catch degenerate scatter direction
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.point, scatter_direction, ray.depth - 1);
        (self.albedo, Some(scattered))
    }
}

pub struct Metal {
    albedo: LinSrgba,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: LinSrgba, fuzz: f32) -> Self {
        Self { albedo, fuzz, }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> (LinSrgba, Option<Ray>) {
        let reflected = reflect(ray.direction, hit.normal).normalize();
        let scattered = Ray::new(
            hit.point,
            reflected + self.fuzz * random_on_unit_sphere(),
            ray.depth - 1,
        );

        if scattered.direction.dot(hit.normal) > 0.0 {
            (self.albedo, Some(scattered))
        } else {
            (LinSrgba::new(0.0, 0.0, 0.0, 1.0), None)
        }
    }
}

pub struct Emission {
    color: LinSrgba,
    strength: f32,
}

impl Emission {
    pub fn new(color: LinSrgba, strength: f32) -> Self {
        Self { color, strength, }
    }
}

impl Material for Emission {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> (LinSrgba, Option<Ray>) {
        (self.color * self.strength, None)
    }
}
