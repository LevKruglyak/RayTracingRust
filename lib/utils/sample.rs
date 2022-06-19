use std::ops::RangeInclusive;

use cgmath::InnerSpace;
use derive_new::new;
use rand::{distributions::uniform::SampleRange, thread_rng, Rng};

use super::{aabb::AABB, types::*};

/// Uniformly samples vectors in a axis aligned cube region
#[derive(new, Clone, Copy)]
pub struct CubeSampler {
    aabb: AABB,
}

impl CubeSampler {
    pub fn from_range(range: RangeInclusive<Float>) -> Self {
        Self {
            aabb: AABB {
                min: Vec3::new(*range.start(), *range.start(), *range.start()),
                max: Vec3::new(*range.end(), *range.end(), *range.end()),
            },
        }
    }
}

impl SampleRange<Vec3> for CubeSampler {
    fn sample_single<R: rand::RngCore + ?Sized>(self, rng: &mut R) -> Vec3 {
        Vec3::new(
            rng.gen_range(self.aabb.min.x..self.aabb.max.x),
            rng.gen_range(self.aabb.min.y..self.aabb.max.y),
            rng.gen_range(self.aabb.min.z..self.aabb.max.z),
        )
    }

    fn is_empty(&self) -> bool {
        self.aabb.is_empty()
    }
}

#[derive(Default)]
pub struct UnitSphereSurfaceSampler {}

impl SampleRange<Vec3> for UnitSphereSurfaceSampler {
    fn sample_single<R: rand::RngCore + ?Sized>(self, rng: &mut R) -> Vec3 {
        let sampler = CubeSampler::from_range(-1.0..=1.0);
        loop {
            let vec = sampler.sample_single(rng);
            if vec.magnitude2() <= 1.0 {
                return vec.normalize();
            }
        }
    }

    fn is_empty(&self) -> bool {
        false
    }
}

pub fn sample_unit_sphere_surface() -> Vec3 {
    UnitSphereSurfaceSampler::default().sample_single(&mut thread_rng())
}
