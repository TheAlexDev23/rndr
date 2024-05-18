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
        object_manager
            .get_object(self.object.expect("Owner object not set"))
            .unwrap()
    }
    pub fn get_mesh<'a>(&'a self, object_manager: &'a ObjectManager) -> &'a MeshRenderable {
        self.get_object(object_manager).component().unwrap()
    }
    pub fn get_transform<'a>(&'a self, object_manager: &'a ObjectManager) -> &'a Transform {
        self.get_object(object_manager).component().unwrap()
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
            .expect("No owner object defined for this component")
            .component::<MeshRenderable>()
            .expect("Referenced object does not have a mesh");

        let transform = object_manager
            .get_object(self.object.unwrap())
            .unwrap()
            .component::<Transform>()
            .expect("Referenced object does not have a transform");

        for triangle in &mesh.triangles {
            let mut a_v = mesh.vertices[triangle[0]];
            let mut b_v = mesh.vertices[triangle[1]];
            let mut c_v = mesh.vertices[triangle[2]];

            a_v.position = a_v.position.rotate(transform.rotation);
            b_v.position = b_v.position.rotate(transform.rotation);
            c_v.position = c_v.position.rotate(transform.rotation);

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
        let self_mesh_center = self.get_mesh(object_manager).center;
        let self_position = self.get_transform(object_manager).position;
        let other = other.get_object(object_manager);

        self_mesh.vertices.par_iter().find_map_first(|vertex| {
            let center_vertex_distance = (vertex.position - self_mesh_center).mag();
            let dir = (vertex.position - self_mesh_center).norm();
            let ray = ObjectIntersectionRay {
                dir,
                start: self_mesh_center + self_position,
                max_distance: None,
                object: other,
            };

            let intersects = ray.cast(object_manager);
            // It is verified that the 2 objects are intersecting eachother
            if intersects
                .iter()
                .filter(|hit| hit.distance >= center_vertex_distance)
                .count()
                % 2
                != 0
            {
                // For now we return the first found vertex intersection. Which given that the tim step is not infinetely small
                // this result is probably not the first actual vertex intersection that happened within the last collision check.
                // In the future the first intersection vertex could be calculated. But I do not know how by now.
                return Some(
                    intersects
                        .into_iter()
                        .reduce(|a, b| {
                            if b.distance > center_vertex_distance {
                                a
                            } else if a.distance < b.distance {
                                a
                            } else {
                                b
                            }
                        })
                        .unwrap()
                        .vertex,
                );
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
