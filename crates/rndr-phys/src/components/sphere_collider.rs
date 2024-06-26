use std::any::TypeId;

use rndr_core::default_components::Transform;
use rndr_core::object::{Component, ObjectManager};

use rndr_math::prelude::V3;

use crate::traits::collidable::IntersectionPoint;
use crate::traits::{Collidable, HitInfo, Raycastable};

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
    fn get_all_ray_intersections(
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
            vec![(-b) / (2.0 * a)]
        } else if discr < 0.0 {
            Vec::new()
        } else {
            let sqrt = discr.sqrt();
            vec![(-b + sqrt) / (2.0 * a), (-b - sqrt) / (2.0 * a)]
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
            let position = start + dir * hit;
            let normal = (position - self_position).norm();
            ret.push(HitInfo {
                distance: hit,
                position,
                normal,
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
    ) -> Option<IntersectionPoint> {
        todo!()
    }

    fn intersects_sphere(
        &self,
        other: &SphereCollider,
        object_manager: &ObjectManager,
    ) -> Option<IntersectionPoint> {
        let self_position = object_manager
            .get_object(self.owner.unwrap())
            .component::<Transform>()
            .position;

        let other_position = object_manager
            .get_object(other.owner.unwrap())
            .component::<Transform>()
            .position;

        if (self_position - other_position).mag() <= self.radius + other.radius {
            return Some(IntersectionPoint {
                normal: (other_position - self_position).norm(),
                position: self_position + (other_position - self_position) / 2.0,
            });
        } else {
            None
        }
    }
}
