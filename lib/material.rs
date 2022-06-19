use crate::{
    color::Color,
    utils::{
        math::{near_zero, reflect, refract},
        ray::Ray,
        sample::sample_unit_sphere_surface,
        types::Float,
    },
};
use cgmath::InnerSpace;
use derive_new::new;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::ray::HitRecord;

#[typetag::serde(tag = "type")]
pub trait Material: Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> (Color, Option<Ray>);
}

#[derive(Serialize, Deserialize)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

#[typetag::serde]
impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> (Color, Option<Ray>) {
        let mut scatter_direction = hit.normal + sample_unit_sphere_surface();

        if near_zero(scatter_direction) {
            // Catch degenerate scatter direction
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.point, scatter_direction);
        (self.albedo, Some(scattered))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Metal {
    albedo: Color,
    fuzz: Float,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: Float) -> Self {
        Self { albedo, fuzz }
    }
}

#[typetag::serde]
impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> (Color, Option<Ray>) {
        let reflected = reflect(ray.direction, hit.normal).normalize();
        let scattered = Ray::new(
            hit.point,
            reflected + self.fuzz * sample_unit_sphere_surface(),
        );

        if scattered.direction.dot(hit.normal) > 0.0 {
            (self.albedo, Some(scattered))
        } else {
            (Color::new(0.0, 0.0, 0.0), None)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Emission {
    color: Color,
}

impl Emission {
    pub fn new(color: Color, strength: Float) -> Self {
        Self {
            color: color * strength,
        }
    }
}

#[typetag::serde]
impl Material for Emission {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> (Color, Option<Ray>) {
        (self.color, None)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Dielectric {
    ir: Float,
}

impl Dielectric {
    pub fn new(ir: Float) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: Float, idx: Float) -> Float {
        // Schlick's approximation for reflectance
        let mut r0 = (1.0 - idx) / (1.0 + idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * Float::powi(1.0 - cosine, 5)
    }
}

#[typetag::serde]
impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> (Color, Option<Ray>) {
        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = Float::min(hit.normal.dot(-unit_direction), 1.0);
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
            Some(Ray::new(hit.point, direction)),
        )
    }
}

#[derive(new, Serialize, Deserialize)]
pub struct MixMaterial {
    first: Box<dyn Material>,
    second: Box<dyn Material>,
    factor: Float,
}

#[typetag::serde]
impl Material for MixMaterial {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> (Color, Option<Ray>) {
        if thread_rng().gen_range(0.0..1.0) >= self.factor {
            self.first.scatter(ray, hit)
        } else {
            self.second.scatter(ray, hit)
        }
    }
}
