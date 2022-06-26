use super::{ray::Ray, types::*};
use serde::{Deserialize, Serialize};

pub trait Bounded {
    fn bounds(&self) -> AABB;
}

#[derive(Serialize, Deserialize, Clone, Copy)]
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

    pub fn from_point(point: Vec3) -> Self {
        Self {
            min: point,
            max: point,
        }
    }

    pub fn centroid(&self) -> Vec3 {
        (self.min + self.max) / 2.0
    }

    pub fn dimensions(&self) -> Vec3 {
        Vec3::new(
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }

    /// Returns a bounding box surrouding two bounding boxes
    pub fn surround(first: Self, second: Self) -> Self {
        Self {
            min: Vec3::new(
                first.min.x.min(second.min.x),
                first.min.y.min(second.min.y),
                first.min.z.min(second.min.z),
            ),
            max: Vec3::new(
                first.max.x.max(second.max.x),
                first.max.y.max(second.max.y),
                first.max.z.max(second.max.z),
            ),
        }
    }

    /// Ensures that no coordinate dimension of the bounding box is below a given epsilon
    pub fn epsilon_expand(bounds: Self, epsilon: f32) -> Self {
        let dimensions = bounds.dimensions();
        let centroid = bounds.centroid();
        let mut result = bounds.clone();

        if dimensions.x < epsilon {
            result.min.x = centroid.x - epsilon;
            result.max.x = centroid.x + epsilon;
        }

        if dimensions.y < epsilon {
            result.min.y = centroid.y - epsilon;
            result.max.y = centroid.y + epsilon;
        }

        if dimensions.z < epsilon {
            result.min.z = centroid.z - epsilon;
            result.max.z = centroid.z + epsilon;
        }

        result
    }

    /// Returns true if the ray hits the bounding box
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

impl Default for AABB {
    fn default() -> Self {
        Self {
            min: Vec3::new(0.0, 0.0, 0.0),
            max: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}
