use cgmath::{InnerSpace, Vector3};
use std::ops::{Add, Mul};

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction * t
    }

    pub fn vertical_grad<T: Add<Output = T> + Mul<f32, Output = T>>(&self, top: T, bottom: T) -> T {
        let t = 0.5 * (self.direction.normalize().y + 1.0);
        top * (1.0 - t) + bottom * t
    }
}
