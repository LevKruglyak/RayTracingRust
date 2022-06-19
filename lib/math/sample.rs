use std::ops::RangeInclusive;

use cgmath::InnerSpace;
use derive_new::new;
use rand::{distributions::uniform::SampleRange, Rng};

use super::{aabb::AABB, shapes::{Sphere, Shape}, types::*};

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

/// Samples vectors inside of a sphere volume
pub struct SphereVolumeSampler {
    sphere: Sphere,
}

impl SphereVolumeSampler {
    pub fn unit_sphere() -> Self {
        Self {
            sphere: Sphere::unit_sphere(),
        }
    }
}

impl SampleRange<Vec3> for SphereVolumeSampler {
    fn sample_single<R: rand::RngCore + ?Sized>(self, rng: &mut R) -> Vec3 {
        let sampler = CubeSampler::new(self.sphere.bounds());
        loop {
            let vec = sampler.sample_single(rng);
            if vec.magnitude() < self.sphere.radius {
                return vec;
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.sphere.is_empty()
    }
}

/// Samples
struct UnitSphereSurfaceSampler {}

impl SampleRange<Vec3> for UnitSphereSurfaceSampler {
    fn sample_single<R: rand::RngCore + ?Sized>(self, rng: &mut R) -> Vec3 {
        let unit_sphere_sampler = SphereVolumeSampler::unit_sphere();
        unit_sphere_sampler.sample_single(rng).normalize()
    }

    fn is_empty(&self) -> bool {
        false
    }
}
