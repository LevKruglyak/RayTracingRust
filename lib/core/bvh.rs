use rand::{thread_rng, Rng};

use crate::core::traits::Hittable;
use crate::utils::{
    aabb::AABB,
    ray::{HitRecord, Ray},
    types::Float,
};

use super::scene::MaterialHandle;

/// Acceleration strcture for faster ray-scene intersections
pub struct BvhTree<'s, S> {
    scene: &'s S,
    root: BvhNode,
}

pub trait BoundsCollection: Sync {
    fn hit(
        &self,
        handle: u32,
        ray: &Ray,
        tmin: Float,
        tmax: Float,
    ) -> Option<HitRecord<MaterialHandle>>;
    fn bounds(&self, handle: u32) -> AABB;
    fn objects(&self) -> Vec<u32>;
}

impl<'s, S> BvhTree<'s, S>
where
    S: BoundsCollection,
{
    /// Build a Bvh tree from a scene
    pub fn build(scene: &'s S) -> Self {
        Self {
            scene,
            root: BvhNode::from_list(&mut scene.objects(), scene),
        }
    }
}

impl<S> Hittable for BvhTree<'_, S>
where
    S: BoundsCollection,
{
    fn hit(&self, ray: &Ray, tmin: Float, tmax: Float) -> Option<HitRecord<MaterialHandle>> {
        self.root.hit(ray, tmin, tmax, self.scene)
    }
}

pub enum BvhNode {
    None,
    Object(u32),
    Split(AABB, Box<BvhNode>, Box<BvhNode>),
}

impl BvhNode {
    pub fn from_list<S>(objects: &mut Vec<u32>, scene: &S) -> Self
    where
        S: BoundsCollection,
    {
        match objects.len() {
            0 => BvhNode::None,
            1 => BvhNode::Object(objects[0]),
            _ => {
                // Calculate bounds for the whole node
                let bounds = objects
                    .iter()
                    .map(|object| scene.bounds(*object))
                    .reduce(|a, b| AABB::surround(a, b))
                    .unwrap_or_default();

                // See which axis has the greatest variance among centroids
                let centroids = objects
                    .iter()
                    .map(|object| AABB::from_point(scene.bounds(*object).centroid()))
                    .reduce(|a, b| AABB::surround(a, b))
                    .unwrap_or_default();

                let spread = centroids.max - centroids.min;
                let axis = if spread.x > spread.y && spread.x > spread.z {
                    0
                } else if spread.y > spread.x && spread.y > spread.z {
                    1
                } else {
                    2
                };

                // Sort objects by laying bounding boxes along an axis
                match axis {
                    0 => {
                        objects.sort_by(|a, b| {
                            scene
                                .bounds(*a)
                                .centroid()
                                .x
                                .partial_cmp(&scene.bounds(*b).centroid().x)
                                .unwrap()
                        });
                    },
                    1 => {
                        objects.sort_by(|a, b| {
                            scene
                                .bounds(*a)
                                .centroid()
                                .y
                                .partial_cmp(&scene.bounds(*b).centroid().y)
                                .unwrap()
                        });
                    },
                    _ => {
                        objects.sort_by(|a, b| {
                            scene
                                .bounds(*a)
                                .centroid()
                                .z
                                .partial_cmp(&scene.bounds(*b).centroid().z)
                                .unwrap()
                        });
                    },
                }

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

    pub fn hit<S>(
        &self,
        ray: &Ray,
        tmin: Float,
        tmax: Float,
        scene: &S,
    ) -> Option<HitRecord<MaterialHandle>>
    where
        S: BoundsCollection,
    {
        match self {
            BvhNode::Object(handle) => {
                return scene.hit(*handle, ray, tmin, tmax);
            }
            BvhNode::Split(bounds, left, right) => {
                if bounds.hit(ray, tmin, tmax) {
                    let hit_left = left.hit(ray, tmin, tmax, scene);
                    let hit_right = right.hit(ray, tmin, tmax, scene);

                    return merge_optionals(hit_left, hit_right);
                }
            }
            _ => {}
        }

        None
    }
}

/// Commonly used to merge hit record results in a Bvh tree
#[inline]
fn merge_optionals<H>(hit_left: Option<H>, hit_right: Option<H>) -> Option<H>
where
    H: PartialOrd,
{
    match (hit_left, hit_right) {
        (Some(record_left), Some(record_right)) => {
            if record_left < record_right {
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

// Compressed Bvh tree representation
// pub struct LinearBvhTree<'a> {
//     scene: &'a Scene,
//     nodes: Vec<LinearBvhNode>,
// }

// enum LinearBvhNode {
//     Object(AABB, ObjectHandle),
//     Split(AABB, u32, u32),
// }

// impl<'a> LinearBvhTree<'a> {
//     pub fn flatten(bvh: BvhTree<'a>) -> Self {
//         let mut nodes = Vec::new();

//         fn flatten_internal(bvh_node: BvhNode, start: u32,) -> u32 {

//         }

//         Self { scene: bvh.scene, nodes, }
//     }
// }

// impl Hittable for LinearBvhTree<'_> {
//     fn hit(&self, ray: &Ray, tmin: Float, tmax: Float) -> Option<HitRecord<MaterialHandle>> {
//         if self.nodes.is_empty() {
//             return None;
//         }

//         let mut node = &self.nodes[0];
//         loop {
//             match node {
//                 LinearBvhNode::Object(bounds, handle) => {
//                     if bounds.hit(ray, tmin, tmax) {
//                         return self.scene.object(*handle).hit(ray, tmin, tmax);
//                     }
//                 }
//                 LinearBvhNode::Split(bounds, left, right) => {
//                     if bounds.hit(ray, tmin, tmax) {
//                         let left = &self.nodes[*left as usize];
//                         let right = &self.nodes[*right as usize];

//                         return None;

// //                         match (left, right) => {

// //                         }

//                         // let hit_left = left.hit(ray, tmin, tmax, scene);
//                         // let hit_right = right.hit(ray, tmin, tmax, scene);

//                         // match (hit_left, hit_right) {
//                         //     (Some(record_left), Some(record_right)) => {
//                         //         if record_left.t < record_right.t {
//                         //             return Some(record_left);
//                         //         } else {
//                         //             return Some(record_right);
//                         //         }
//                         //     }
//                         //     (Some(record_left), None) => {
//                         //         return Some(record_left);
//                         //     }
//                         //     (None, Some(record_right)) => {
//                         //         return Some(record_right);
//                         //     }
//                         //     _ => {
//                         //         return None;
//                         //     }
//                         // }
//                     }
//                 }
//                 _ => {}
//             }
//         }
//     }
// }

// // impl LinearBvhNode {
// //     fn hit(&self, ray: &Ray, tmin: Float, tmax: Float, scene: &Scene) -> Option<HitRecord<MaterialHandle>> {
// //         match self {
// //             LinearBvhNode::Object(bounds, handle) => {
// //                 if bounds.hit(ray, tmin, tmax) {
// //                     return scene.object(*handle).hit(ray, tmin, tmax);
// //                 }

// //                 None
// //             },
// //             LinearBvhNode::Split(bounds, left, right) => {

// //             },
// //         }
// //     }
// // }
