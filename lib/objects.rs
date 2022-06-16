use crate::{
    ray::{HitRecord, Hittable, Ray},
    scene::MaterialHandle,
};
use cgmath::{InnerSpace, Vector3};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, new)]
pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    material: MaterialHandle,
}

#[typetag::serde]
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, limits: (f32, f32)) -> Option<HitRecord> {
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
        if root < limits.0 || limits.1 < root {
            root = (-half_b + sqrtd) / a;
            if root < limits.0 || limits.1 < root {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;

        Some(HitRecord::new(point, normal, root, ray, self.material))
    }
}
