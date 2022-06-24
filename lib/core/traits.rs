use crate::{
    gui::gui::Editable,
    utils::{
        aabb::Bounded,
        color::Color,
        ray::{HitRecord, Ray},
        types::Float,
    },
};

use super::scene::MaterialHandle;

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, tmin: Float, tmax: Float) -> Option<HitRecord<MaterialHandle>>;
}

#[typetag::serde(tag = "type")]
pub trait Object: Sync + Hittable + Bounded {
    fn material(&self) -> MaterialHandle;
}

#[typetag::serde(tag = "type")]
pub trait Material: Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord<MaterialHandle>) -> (Color, Option<Ray>);
}

#[typetag::serde(tag = "type")]
pub trait Background: Sync + Editable {
    fn sample(&self, ray: &Ray) -> Color;
}
