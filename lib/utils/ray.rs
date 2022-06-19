use std::ops::{Add, Mul};

use cgmath::InnerSpace;
use derive_new::new;

use super::types::*;

#[derive(new, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: Float) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn vertical_grad<T: Add<Output = T> + Mul<Float, Output = T>>(
        &self,
        top: T,
        bottom: T,
    ) -> T {
        let t = 0.5 * (self.direction.normalize().y + 1.0);
        top * (1.0 - t) + bottom * t
    }
}
