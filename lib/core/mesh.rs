use crate::utils::{
    aabb::{Bounded, AABB},
    ray::{HitRecord, Ray},
    types::{Float, Vec3},
};
use cgmath::InnerSpace;
use obj::{load_obj, Obj, Vertex};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

use super::{
    bvh::{BoundsCollection, BvhNode},
    scene::MaterialHandle,
    traits::{Hittable, Object},
};

#[derive(Serialize, Deserialize)]
pub struct Triangle {
    vertices: [u32; 3],
    normal: Vec3,
}

pub struct Mesh {
    /// Vertex buffer
    vertices: Vec<Vec3>,
    bounds: AABB,
    triangles: Vec<Triangle>,
    bvh_root: BvhNode,
    material: MaterialHandle,
}

impl Serialize for Mesh {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        loop { panic!("bruh") }
    }
}

impl<'de> Deserialize<'de> for Mesh {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        loop { panic!("bruh") }
    }
}

impl Mesh {
    pub fn from_file(path: &str, material: MaterialHandle) -> Self {
        let obj: Obj<Vertex, u32> = load_obj(BufReader::new(File::open(path).unwrap())).unwrap();
        let mut vertices: Vec<Vec3> = Vec::new();

        for vertex in obj.vertices {
            vertices.push(Vec3::new(
                vertex.position[0],
                vertex.position[1],
                vertex.position[2],
            ));
        }

        Self::from_buffers(vertices, obj.indices, material)
    }

    pub fn from_buffers(vertices: Vec<Vec3>, indices: Vec<u32>, material: MaterialHandle) -> Self {
        let mut triangles = Vec::<Triangle>::new();

        for triangle in indices.chunks_exact(3) {
            let e1 = vertices[triangle[0] as usize] - vertices[triangle[1] as usize];
            let e2 = vertices[triangle[2] as usize] - vertices[triangle[1] as usize];
            let normal = e1.cross(e2).normalize();

            triangles.push(Triangle {
                vertices: [triangle[0], triangle[1], triangle[2]],
                normal,
            })
        }

        // Calculate bounding box
        let mut bounds = AABB::default();
        for vertex in &vertices[..] {
            bounds = AABB::surround(
                bounds,
                AABB {
                    min: *vertex,
                    max: *vertex,
                },
            );
        }

        Self {
            vertices,
            triangles,
            material,
            bvh_root: BvhNode::None,
            bounds,
        }
    }

    pub fn build_bvh(&mut self) {
        self.bvh_root = BvhNode::from_list(&mut self.objects(), self);
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, tmin: Float, tmax: Float) -> Option<HitRecord<MaterialHandle>> {
        return self.bvh_root.hit(ray, tmin, tmax, self)

//        let mut result = None;
//        let mut closest_so_far = tmax;

//        for triangle in &self.triangles {
//            //if object.bounds().hit(ray, tmin, tmax) {
//            if let Some(hit) = triangle.hit(ray, tmin, closest_so_far, self.material, &self) {
//                if closest_so_far > hit.t {
//                    closest_so_far = hit.t;
//                    result = Some(hit);
//                }
//            }
//            //}
//        }

//        result
    }
}

impl Triangle {
    fn hit(
        &self,
        ray: &Ray,
        tmin: Float,
        _: Float,
        material: MaterialHandle,
        mesh: &Mesh,
    ) -> Option<HitRecord<MaterialHandle>> {
        let v0 = mesh.vertices[self.vertices[0] as usize];
        let v1 = mesh.vertices[self.vertices[1] as usize];
        let v2 = mesh.vertices[self.vertices[2] as usize];

        // Edges
        let e1 = v1 - v0;
        let e2 = v2 - v0;

        let h = ray.direction.cross(e2);
        let a = e1.dot(h);

        if a > -tmin && a < tmin {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(e1);
        let v = f * ray.direction.dot(q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * e2.dot(q);

        if t > tmin {
            Some(HitRecord::new(ray.at(t), self.normal, t, ray, material))
        } else {
            None
        }
    }
}

/// Make it possible to create Bvh tree for mesh
impl BoundsCollection for Mesh {
    fn bounds(&self, handle: u32) -> AABB {
        let triangle = &self.triangles[handle as usize];
        AABB::surround(
            AABB::from_point(self.vertices[triangle.vertices[0] as usize]),
            AABB::surround(
                AABB::from_point(self.vertices[triangle.vertices[1] as usize]),
                AABB::from_point(self.vertices[triangle.vertices[2] as usize]),
            ),
        )
    }

    fn objects(&self) -> Vec<u32> {
        self.triangles.iter().enumerate().map(|(index, _)| index as u32).collect()
    }

    fn hit(&self, handle: u32, ray: &Ray, tmin: Float, tmax: Float) -> Option<HitRecord<MaterialHandle>> {
        self.triangles[handle as usize].hit(ray, tmin, tmax, self.material, self)
    }
}

impl Bounded for Mesh {
    fn bounds(&self) -> AABB {
        self.bounds
    }
}

#[typetag::serde]
impl Object for Mesh {}