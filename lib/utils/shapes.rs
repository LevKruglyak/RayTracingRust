use super::{
    aabb::{Bounded, AABB},
    types::{Float, Vec3},
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: Float,
}

impl Sphere {
    pub fn unit_sphere() -> Self {
        Self {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.radius <= 0.0
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