use crate::utils::{ray::Ray, types::*};
use serde::{Deserialize, Serialize};

use crate::{
    backgrounds::UniformBackground, core::camera::Camera, utils::color::Color,
    utils::ray::HitRecord,
};

use super::{
    bvh::BvhTree,
    traits::{Background, Hittable, Material, Object},
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RenderMode {
    Full,
    Clay,
    Random,
    Normal,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RenderSettings {
    pub samples_per_pixel: u32,
    pub max_ray_depth: u8,
    pub enable_multithreading: bool,
    pub enable_bvh_tree: bool,
    pub mode: RenderMode,
}

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub settings: RenderSettings,
    pub background: Box<dyn Background>,
    // Temporary
    pub objects: Vec<Box<dyn Object>>,
    materials: Vec<Box<dyn Material>>,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            camera: Camera {
                lookfrom: Vec3::new(0.0, 0.0, 0.0),
                lookat: Vec3::new(0.0, 0.0, -1.0),
                vertical: Vec3::new(0.0, 1.0, 0.0),
                vertical_fov: 90.0,
                aspect_ratio: 1.0,
            },
            settings: RenderSettings {
                samples_per_pixel: 5,
                max_ray_depth: 6,
                enable_multithreading: true,
                enable_bvh_tree: true,
                mode: RenderMode::Full,
            },
            background: Box::new(UniformBackground::new(Color::new(0.8, 0.8, 0.8))),
            objects: Vec::new(),
            materials: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MaterialHandle(pub usize);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ObjectHandle(pub usize);

impl Scene {
    pub fn from_file(path: &str) -> Self {
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()[..]).unwrap()
    }

    pub fn new(settings: RenderSettings, camera: Camera, background: Box<dyn Background>) -> Self {
        Self {
            camera,
            settings,
            background,
            objects: Vec::new(),
            materials: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Object>) -> ObjectHandle {
        self.objects.push(object);
        ObjectHandle(self.objects.len() - 1)
    }

    #[inline]
    pub fn object(&self, object: ObjectHandle) -> &Box<dyn Object> {
        // SAFETY: Shouln't be out of bounds because ObjectHandle only constructed
        // in this impl as an index in a grow-only vector
        &self.objects[object.0]
    }

    pub fn add_material(&mut self, material: Box<dyn Material>) -> MaterialHandle {
        self.materials.push(material);
        MaterialHandle(self.materials.len() - 1)
    }

    #[inline]
    pub fn material(&self, material: MaterialHandle) -> &Box<dyn Material> {
        // SAFETY: Shouln't be out of bounds because ObjectHandle only constructed
        // in this impl as an index in a grow-only vector
        &self.materials[material.0]
    }

    pub fn build_bvh(&self) -> BvhTree {
        BvhTree::build(&self)
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, tmin: Float, tmax: Float) -> Option<HitRecord<MaterialHandle>> {
        let mut result = None;
        let mut closest_so_far = tmax;

        for object in &self.objects {
            //if object.bounds().hit(ray, tmin, tmax) {
            if let Some(hit) = object.hit(ray, tmin, closest_so_far) {
                closest_so_far = hit.t;
                result = Some(hit);
            }
            //}
        }

        result
    }
}
