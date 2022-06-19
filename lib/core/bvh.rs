use crate::core::traits::Hittable;
use crate::utils::{
    aabb::AABB,
    ray::{HitRecord, Ray},
    types::Float,
};

use super::scene::{ObjectHandle, Scene, MaterialHandle};

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
    fn hit(&self, ray: &Ray, tmin: Float, tmax: Float) -> Option<HitRecord<MaterialHandle>> {
        self.root.hit(ray, tmin, tmax, self.scene)
    }
}

/// Internal trait
trait BvhHittable {
    fn hit(&self, ray: &Ray, tmin: Float, tmax: Float, scene: &Scene) -> Option<HitRecord<MaterialHandle>>;
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
    fn hit(&self, ray: &Ray, tmin: Float, tmax: Float, scene: &Scene) -> Option<HitRecord<MaterialHandle>> {
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
