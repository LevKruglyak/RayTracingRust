use crate::color::Color;
use cgmath::InnerSpace;
use derive_new::new;
use rand::{thread_rng, Rng};

use crate::{
    ray::{HitRecord, Ray},
    utils::{near_zero, random_on_unit_sphere, reflect, refract},
};

pub trait Material: Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> (Color, Option<Ray>);
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> (Color, Option<Ray>) {
        let mut scatter_direction = hit.normal + random_on_unit_sphere();

        if near_zero(scatter_direction) {
            // Catch degenerate scatter direction
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.point, scatter_direction, ray.depth + 1);
        (self.albedo, Some(scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> (Color, Option<Ray>) {
        let reflected = reflect(ray.direction, hit.normal).normalize();
        let scattered = Ray::new(
            hit.point,
            reflected + self.fuzz * random_on_unit_sphere(),
            ray.depth + 1,
        );

        if scattered.direction.dot(hit.normal) > 0.0 {
            (self.albedo, Some(scattered))
        } else {
            (Color::new(0.0, 0.0, 0.0), None)
        }
    }
}

pub struct Emission {
    color: Color,
}

impl Emission {
    pub fn new(color: Color, strength: f32) -> Self {
        Self {
            color: color * strength,
        }
    }
}

impl Material for Emission {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> (Color, Option<Ray>) {
        (self.color, None)
    }
}

pub struct Dielectric {
    ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f32, idx: f32) -> f32 {
        // Schlick's approximation for reflectance
        let mut r0 = (1.0 - idx) / (1.0 + idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> (Color, Option<Ray>) {
        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = f32::min(hit.normal.dot(-unit_direction), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = (refraction_ratio * sin_theta) > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio)
                > thread_rng().gen_range(0.0..1.0)
        {
            reflect(unit_direction, hit.normal)
        } else {
            refract(unit_direction, hit.normal, refraction_ratio)
        };

        (
            Color::new(1.0, 1.0, 1.0),
            Some(Ray::new(hit.point, direction, ray.depth + 1)),
        )
    }
}

#[derive(new)]
pub struct MixMaterial {
    first: Box<dyn Material>,
    second: Box<dyn Material>,
    factor: f32,
}

impl Material for MixMaterial {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> (Color, Option<Ray>) {
        if thread_rng().gen_range(0.0..1.0) >= self.factor {
            self.first.scatter(ray, hit)
        } else {
            self.second.scatter(ray, hit)
        }
    }
}
