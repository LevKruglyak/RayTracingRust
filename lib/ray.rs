use cgmath::{InnerSpace, Vector3};
use std::{
    ops::{Add, Mul},
    rc::Rc,
};

use crate::material::Material;

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub depth: u8,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>, depth: u8) -> Self {
        Self {
            origin,
            direction,
            depth,
        }
    }

    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction * t
    }

    pub fn vertical_grad<T: Add<Output = T> + Mul<f32, Output = T>>(&self, top: T, bottom: T) -> T {
        let t = 0.5 * (self.direction.normalize().y + 1.0);
        top * (1.0 - t) + bottom * t
    }
}

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        point: Vector3<f32>,
        outward_normal: Vector3<f32>,
        t: f32,
        ray: &Ray,
        material: Rc<dyn Material>,
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

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
