use cgmath::InnerSpace;

use crate::{
    scene::{MaterialHandle, Object},
    utils::{
        aabb::{Bounded, AABB},
        ray::Ray,
        types::{Float, Vec3},
    },
};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: Float,
    pub front_face: bool,
    pub material: MaterialHandle,
}

impl HitRecord {
    pub fn new(
        point: Vec3,
        outward_normal: Vec3,
        t: Float,
        ray: &Ray,
        material: MaterialHandle,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, tmin: Float, tmax: Float) -> Option<HitRecord>;
}
