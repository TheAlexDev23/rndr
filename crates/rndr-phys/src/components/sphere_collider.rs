use std::any::TypeId;

use rndr_core::default_components::Transform;
use rndr_core::object::{Component, ObjectManager};

use rndr_math::prelude::{Vertex, V3};

use crate::collision::Collidable;
use crate::raycast::{HitInfo, Raycastable};

use super::MeshCollider;

#[derive(Debug)]
pub struct SphereCollider {
    owner: Option<u64>,
    radius: f32,
}

impl SphereCollider {
    pub fn new(radius: f32) -> SphereCollider {
        SphereCollider {
            owner: None,
            radius,
        }
    }
}

impl Component for SphereCollider {
    fn get_type(&self) -> TypeId {
        TypeId::of::<SphereCollider>()
    }
    fn on_added(&mut self, object: u64) {
        self.owner = Some(object)
    }
}

impl Raycastable for SphereCollider {
    fn ray_intersects(
        &self,
        object_manager: &ObjectManager,
        start: V3,
        dir: V3,
        max_distance: Option<f32>,
    ) -> Vec<HitInfo> {
        let self_position = object_manager
            .get_object(self.owner.expect("Owner not set"))
            .component::<Transform>()
            .position;

        let n = start - self_position;
        let a = dir.hadamard_product(dir).all_elements_sum();
        let b = (n.hadamard_product(dir) * 2.0).all_elements_sum();
        let c = (n.hadamard_product(n)).all_elements_sum() - self.radius.powi(2);

        let discr = b.powi(2) - (4.0 * a * c);
        let hits = if discr.abs() < f32::EPSILON {
            vec![(-1.0 * b) / (2.0 * a)]
        } else if discr < 0.0 {
            Vec::new()
        } else {
            let sqrt = discr.sqrt();
            vec![(-1.0 * b + sqrt) / (2.0 * a), (-1.0 * b - sqrt) / (2.0 * a)]
        };

        let mut ret = Vec::new();
        for hit in hits {
            if hit < 0.0 {
                continue;
            }
            if let Some(max_distance) = max_distance {
                if hit > max_distance {
                    continue;
                }
            }
            ret.push(HitInfo {
                distance: hit,
                vertex: Vertex {
                    position: start + dir * hit,
                    color: [255; 3],
                    ..Default::default()
                },
            })
        }
        ret
    }
}

impl Collidable for SphereCollider {
    fn intersects_mesh(
        &self,
        _other: &MeshCollider,
        _object_manager: &ObjectManager,
    ) -> Option<Vertex> {
        todo!()
    }

    fn intersects_sphere(
        &self,
        _other: &SphereCollider,
        _object_manager: &ObjectManager,
    ) -> Option<Vertex> {
        todo!()
    }
}
