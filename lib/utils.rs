use cgmath::{InnerSpace, Vector3};
use rand::{prelude::ThreadRng, Rng};

pub fn random_vector(rng: &mut ThreadRng, min: f32, max: f32) -> Vector3<f32> {
    Vector3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vector3<f32> {
    loop {
        let rvec = random_vector(rng, -1.0, 1.0);
        if rvec.magnitude() < 1.0 {
            return rvec;
        }
    }
}

pub fn random_on_unit_sphere(rng: &mut ThreadRng) -> Vector3<f32> {
    random_in_unit_sphere(rng).normalize()
}
