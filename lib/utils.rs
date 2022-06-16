use std::f32::consts::PI;

use cgmath::{InnerSpace, Vector2, Vector3};
use rand::Rng;

pub fn random_vector(min: f32, max: f32) -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    Vector3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn random_in_unit_sphere() -> Vector3<f32> {
    loop {
        let rvec = random_vector(-1.0, 1.0);
        if rvec.magnitude() < 1.0 {
            return rvec;
        }
    }
}

#[inline]
pub fn random_on_unit_sphere() -> Vector3<f32> {
    random_in_unit_sphere().normalize()
}

pub fn near_zero(vector: Vector3<f32>) -> bool {
    const EPS: f32 = 1.0e-8;
    vector.x.abs() < EPS && vector.y.abs() < EPS && vector.z.abs() < EPS
}

pub fn reflect(a: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
    a - 2.0 * a.dot(n) * n
}

pub fn refract(uv: Vector3<f32>, n: Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
    let cos_theta = f32::min(n.dot(-uv), 1.0);
    let out_perp = etai_over_etat * (uv + cos_theta * n);
    let out_parallel = -(f32::abs(1.0 - out_perp.magnitude2())).sqrt() * n;
    out_perp + out_parallel
}

#[inline]
pub fn to_spherical_coords(v: Vector3<f32>) -> Vector2<f32> {
    Vector2 {
        x: (-v.y).acos(),
        y: f32::atan2(-v.z, v.x) + PI,
    }
}

#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}
