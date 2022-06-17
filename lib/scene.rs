use serde::{Deserialize, Serialize};

use crate::{
    camera::Camera,
    material::Material,
    ray::{HitRecord, Hittable, Ray},
    sky::Background,
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
    pub mode: RenderMode,
}

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub settings: RenderSettings,
    pub background: Box<dyn Background>,
    objects: Vec<Box<dyn Hittable>>,
    materials: Vec<Box<dyn Material>>,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct MaterialHandle(usize);

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ObjectHandle(usize);

impl Scene {
    pub fn new(settings: RenderSettings, camera: Camera, background: Box<dyn Background>) -> Self {
        Self {
            camera,
            settings,
            background,
            objects: Vec::new(),
            materials: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Hittable>) -> ObjectHandle {
        self.objects.push(object);
        ObjectHandle(self.objects.len() - 1)
    }

    #[inline]
    pub fn object(&self, object: ObjectHandle) -> &Box<dyn Hittable> {
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

    pub fn hit(&self, ray: &Ray, limits: (f32, f32)) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = f32::INFINITY;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, (limits.0, closest_so_far)) {
                closest_so_far = hit.t;
                result = Some(hit);
            }
        }

        result
    }
}
