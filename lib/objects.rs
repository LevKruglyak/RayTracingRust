use crate::{
    ray::{HitRecord, Hittable},
    scene::MaterialHandle,
    utils::{
        aabb::{Bounded, AABB},
        ray::Ray,
        types::*,
    },
};
use cgmath::InnerSpace;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, new)]
pub struct Sphere {
    center: Vec3,
    radius: Float,
    material: MaterialHandle,
}

#[typetag::serde]
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: Float, tmax: Float) -> Option<HitRecord> {
        let oc = ray.origin - self.center;

        let a = ray.direction.magnitude2();
        let half_b = oc.dot(ray.direction);
        let c = oc.magnitude2() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in an acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < tmin || tmax < root {
            root = (-half_b + sqrtd) / a;
            if root < tmin || tmax < root {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;

        Some(HitRecord::new(point, normal, root, ray, self.material))
    }
}

impl Bounded for Sphere {
    fn bounds(&self) -> AABB {
        AABB {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        }
    }
}
