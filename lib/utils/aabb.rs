use super::{ray::Ray, types::*};

pub trait Bounded {
    fn bounds(&self) -> AABB;
}

#[derive(Clone, Copy)]
pub struct AABB {
    /// Lower left corner of the AABB
    pub min: Vec3,
    // Upper right corner of the AABB
    pub max: Vec3,
}

impl AABB {
    pub fn is_empty(&self) -> bool {
        self.min.x <= self.max.x && self.min.y <= self.max.y && self.min.z <= self.max.z
    }

    pub fn hit(&self, ray: &Ray, tmin: Float, tmax: Float) -> bool {
        let mut tmin = tmin;
        let mut tmax = tmax;

        // x pass
        let inv_d = 1.0 / ray.direction.x;
        let mut t0 = (self.min.x - ray.origin.x) * inv_d;
        let mut t1 = (self.max.x - ray.origin.x) * inv_d;
        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        if t0 > tmin {
            tmin = t0;
        }
        if t1 < tmax {
            tmax = t1;
        }
        if tmax <= tmin {
            return false;
        }

        // y pass
        let inv_d = 1.0 / ray.direction.y;
        let mut t0 = (self.min.y - ray.origin.y) * inv_d;
        let mut t1 = (self.max.y - ray.origin.y) * inv_d;
        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        if t0 > tmin {
            tmin = t0;
        }
        if t1 < tmax {
            tmax = t1;
        }
        if tmax <= tmin {
            return false;
        }

        // z pass
        let inv_d = 1.0 / ray.direction.z;
        let mut t0 = (self.min.z - ray.origin.z) * inv_d;
        let mut t1 = (self.max.z - ray.origin.z) * inv_d;
        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        if t0 > tmin {
            tmin = t0;
        }
        if t1 < tmax {
            tmax = t1;
        }
        if tmax <= tmin {
            return false;
        }

        true
    }
}
