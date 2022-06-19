use std::sync::Arc;

use crate::{utils::{
    aabb::{Bounded, AABB},
    ray::Ray,
    types::*,
}, sky::UniformBackground};
use serde::{Deserialize, Serialize};

use crate::{
    camera::Camera,
    material::Material,
    color::Color,
    ray::{HitRecord, Hittable},
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
    pub enable_bvh_tree: bool,
    pub mode: RenderMode,
}

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub settings: RenderSettings,
    pub background: Box<dyn Background>,
    objects: Vec<Box<dyn Object>>,
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
pub struct MaterialHandle(usize);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ObjectHandle(usize);

#[typetag::serde(tag = "type")]
pub trait Object: Sync + Hittable + Bounded {}

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
    fn hit(&self, ray: &Ray, tmin: Float, tmax: Float) -> Option<HitRecord> {
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

/// Internal trait
trait BvhHittable {
    fn hit(&self, ray: &Ray, tmin: Float, tmax: Float, scene: &Scene) -> Option<HitRecord>;
}

/// Acceleration strcture for faster ray-scene intersections
pub struct BvhTree<'s> {
    scene: &'s Scene,
    root: BvhNode,
}

impl<'s> BvhTree<'s> {
    /// Build a Bvh tree from a scene
    pub fn build(scene: &'s Scene) -> Self {
        let mut objects: Vec<ObjectHandle> = Vec::new();
        for (index, _) in scene.objects.iter().enumerate() {
            objects.push(ObjectHandle(index));
        }

        Self {
            scene,
            root: BvhNode::from_list(&mut objects, scene),
        }
    }
}

impl Hittable for BvhTree<'_> {
    fn hit(&self, ray: &Ray, tmin: Float, tmax: Float) -> Option<HitRecord> {
        self.root.hit(ray, tmin, tmax, self.scene)
    }
}

enum BvhNode {
    None,
    Object(ObjectHandle),
    Split(AABB, Box<BvhNode>, Box<BvhNode>),
}

impl BvhNode {
    fn from_list(objects: &mut Vec<ObjectHandle>, scene: &Scene) -> Self {
        match objects.len() {
            0 => BvhNode::None,
            1 => BvhNode::Object(objects[0]),
            _ => {
                // Calculate bounds for the whole node
                let bounds = objects
                    .iter()
                    .map(|object| scene.object(*object).bounds())
                    .reduce(|a, b| AABB::surround(a, b))
                    .unwrap_or_default();

                // Sort objects by laying bounding boxes along an axis
                objects.sort_by(|a, b| {
                    scene
                        .object(*a)
                        .bounds()
                        .min
                        .x
                        .partial_cmp(&scene.object(*b).bounds().min.x)
                        .unwrap()
                });

                // Assign first half to left half and second half to right half
                let mut left_list = Vec::new();
                let mut right_list = Vec::new();

                for (index, element) in objects.iter().enumerate() {
                    if index < objects.len() / 2 {
                        left_list.push(*element);
                    } else {
                        right_list.push(*element);
                    }
                }

                // Create the node
                BvhNode::Split(
                    bounds,
                    Box::new(BvhNode::from_list(&mut left_list, scene)),
                    Box::new(BvhNode::from_list(&mut right_list, scene)),
                )
            }
        }
    }
}

impl BvhHittable for BvhNode {
    fn hit(&self, ray: &Ray, tmin: Float, tmax: Float, scene: &Scene) -> Option<HitRecord> {
        match self {
            BvhNode::Object(handle) => {
                return scene.object(*handle).hit(ray, tmin, tmax);
            }
            BvhNode::Split(bounds, left, right) => {
                if bounds.hit(ray, tmin, tmax) {
                    let hit_left = left.hit(ray, tmin, tmax, scene);
                    let hit_right = right.hit(ray, tmin, tmax, scene);

                    match (hit_left, hit_right) {
                        (Some(record_left), Some(record_right)) => {
                            if record_left.t < record_right.t {
                                return Some(record_left);
                            } else {
                                return Some(record_right);
                            }
                        }
                        (Some(record_left), None) => {
                            return Some(record_left);
                        }
                        (None, Some(record_right)) => {
                            return Some(record_right);
                        }
                        _ => {
                            return None;
                        }
                    }
                }
            }
            _ => {}
        }

        None
    }
}
