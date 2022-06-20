use crate::core::traits::Hittable;
use crate::utils::{
    aabb::AABB,
    ray::{HitRecord, Ray},
    types::Float,
};

use super::scene::{MaterialHandle, ObjectHandle, Scene};

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

    fn hit(
        &self,
        ray: &Ray,
        tmin: Float,
        tmax: Float,
        scene: &Scene,
    ) -> Option<HitRecord<MaterialHandle>> {
        match self {
            BvhNode::Object(handle) => {
                return scene.object(*handle).hit(ray, tmin, tmax);
            }
            BvhNode::Split(bounds, left, right) => {
                if bounds.hit(ray, tmin, tmax) {
                    let hit_left = left.hit(ray, tmin, tmax, scene);
                    let hit_right = right.hit(ray, tmin, tmax, scene);

                    return merge_hitrecords(hit_left, hit_right);
                }
            }
            _ => {}
        }

        None
    }
}

#[inline]
fn merge_hitrecords<M>(
    hit_left: Option<HitRecord<M>>,
    hit_right: Option<HitRecord<M>>,
) -> Option<HitRecord<M>> {
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
