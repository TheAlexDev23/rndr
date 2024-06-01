use std::any::TypeId;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rndr_core::default_components::{render::MeshRenderable, Transform};
use rndr_core::object::{Component, Object, ObjectManager};

use rndr_math::prelude::{M3x3, Vertex};
use rndr_math::vector::V3;

use crate::collision::Collidable;
use crate::prelude::HitInfo;
use crate::raycast::ObjectIntersectionRay;
use crate::raycast::Raycastable;

use super::SphereCollider;

#[derive(Default, Debug)]
pub struct MeshCollider {
    object: Option<u64>,
}

impl Component for MeshCollider {
    fn get_type(&self) -> TypeId {
        TypeId::of::<MeshCollider>()
    }

    fn on_added(&mut self, object: u64) {
        self.object = Some(object)
    }
}

impl MeshCollider {
    pub fn get_object<'a>(&'a self, object_manager: &'a ObjectManager) -> &'a Object {
        object_manager.get_object(self.object.expect("Owner object not set"))
    }
    pub fn get_mesh<'a>(&'a self, object_manager: &'a ObjectManager) -> &'a MeshRenderable {
        self.get_object(object_manager).component()
    }
    pub fn get_transform<'a>(&'a self, object_manager: &'a ObjectManager) -> &'a Transform {
        self.get_object(object_manager).component()
    }
}

impl Raycastable for MeshCollider {
    fn ray_intersects(
        &self,
        object_manager: &ObjectManager,
        start: V3,
        dir: V3,
        max_distance: Option<f32>,
    ) -> Vec<HitInfo> {
        let mut ret = Vec::new();

        let mesh = object_manager
            .get_object(self.object.unwrap())
            .component::<MeshRenderable>();

        let transform = object_manager
            .get_object(self.object.unwrap())
            .component::<Transform>();

        for triangle in &mesh.triangles {
            let mut a_v = mesh.vertices[triangle[0]];
            let mut b_v = mesh.vertices[triangle[1]];
            let mut c_v = mesh.vertices[triangle[2]];

            a_v.position = a_v.position.rotate(transform.rotation);
            b_v.position = b_v.position.rotate(transform.rotation);
            c_v.position = c_v.position.rotate(transform.rotation);

            a_v.normal = a_v.normal.rotate(transform.rotation);
            b_v.normal = b_v.normal.rotate(transform.rotation);
            c_v.normal = c_v.normal.rotate(transform.rotation);

            a_v.position += transform.position;
            b_v.position += transform.position;
            c_v.position += transform.position;

            let a = a_v.position;
            let b = b_v.position;
            let c = c_v.position;

            let conversion_matrix = M3x3::new([b - a, c - a, -1.0 * dir]).inv();

            if conversion_matrix.is_none() {
                continue;
            }

            let res = (start - a) * conversion_matrix.unwrap();

            let t = res.z;
            let v = res.x;
            let w = res.y;

            if t < 0.0 || v < 0.0 || w < 0.0 || v + w > 1.0 {
                continue;
            }

            if let Some(max_distance) = max_distance {
                // Apparently making this a one liner is unstable; so 2 nested ifs are required
                if t > max_distance {
                    continue;
                }
            }

            let u = 1.0 - (v + w);
            let vertex = Vertex::interpolate((a_v, u), (b_v, v), (c_v, w));

            ret.push(HitInfo {
                vertex,
                distance: t,
            });
        }
        ret
    }
}

impl Collidable for MeshCollider {
    fn intersects_mesh(
        &self,
        other: &MeshCollider,
        object_manager: &ObjectManager,
    ) -> Option<Vertex> {
        let self_mesh = self.get_mesh(object_manager);
        let self_transform = self.get_transform(object_manager);
        let self_mesh_center = self_mesh.calculate_center(self_transform);
        let other = other.get_object(object_manager);

        self_mesh.vertices.par_iter().find_map_first(|vertex| {
            let mut vertex = vertex.clone();
            vertex.position = vertex.position.rotate(self_transform.rotation);
            vertex.position += self_transform.position;

            let center_vertex_distance = (vertex.position - self_mesh_center).mag();
            let dir = (vertex.position - self_mesh_center).norm();
            let ray = ObjectIntersectionRay {
                dir,
                start: self_mesh_center,
                max_distance: Some(center_vertex_distance),
                object: other,
            };

            let intersects = ray.cast(object_manager);
            if intersects.len() != 0 {
                let mut v = intersects.first().unwrap().vertex;
                v.normal *= -1.0;
                return Some(v);
            }

            None
        })
    }

    fn intersects_sphere(
        &self,
        _other: &SphereCollider,
        _object_manager: &ObjectManager,
    ) -> Option<Vertex> {
        todo!()
    }
}
