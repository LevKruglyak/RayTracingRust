use super::types::*;

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
}
