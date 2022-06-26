use crate::{
    core::{
        scene::MaterialHandle,
        traits::{Hittable, Material, Object},
    },
    utils::{
        aabb::{Bounded, AABB},
        color::Color,
        ray::{HitRecord, Ray},
        sample::sample_unit_sphere_volume,
        types::{Float, Vec3},
    },
};
use cgmath::InnerSpace;
use derive_new::new;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Volume {
    boundary: Box<dyn Object>,
    neg_inv_density: Float,
}

impl Volume {
    pub fn new(boundary: Box<dyn Object>, density: Float) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
        }
    }
}

impl Hittable for Volume {
    fn hit(&self, ray: &Ray, tmin: Float, tmax: Float) -> Option<HitRecord<MaterialHandle>> {
        let mut hit1 = self.boundary.hit(ray, -f32::INFINITY, f32::INFINITY)?;
        let mut hit2 = self.boundary.hit(ray, hit1.t + tmin, f32::INFINITY)?;

        if hit1.t < tmin {
            hit1.t = tmin;
        }

        if hit2.t > tmax {
            hit2.t = tmax;
        }

        if hit1.t >= hit2.t {
            return None;
        }

        if hit1.t < 0.0 {
            hit1.t = 0.0;
        }

        let ray_length = ray.direction.magnitude();
        let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
        let random_float: Float = thread_rng().gen_range(0.0..=1.0);
        let hit_distance = self.neg_inv_density * random_float.ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hit1.t + hit_distance / ray_length;
        Some(HitRecord::new(
            ray.at(t),
            Vec3::new(1.0, 0.0, 0.0),
            t,
            ray,
            self.boundary.material(),
        ))
    }
}

#[derive(new, Serialize, Deserialize)]
pub struct Isotropic {
    color: Color,
}

#[typetag::serde]
impl Material for Isotropic {
    fn scatter(&self, _: &Ray, hit: &HitRecord<MaterialHandle>) -> (Color, Option<Ray>) {
        (
            self.color,
            Some(Ray::new(hit.point, sample_unit_sphere_volume())),
        )
    }
}

impl Bounded for Volume {
    fn bounds(&self) -> AABB {
        self.boundary.bounds()
    }
}

#[typetag::serde]
impl Object for Volume {
    fn material(&self) -> MaterialHandle {
        self.boundary.material()
    }
}
