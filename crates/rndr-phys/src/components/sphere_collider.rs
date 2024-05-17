use std::any::TypeId;

use rndr_core::object::{Component, ObjectManager};

use rndr_math::prelude::{Vertex, V3};

use crate::collision::Collidable;
use crate::raycast::{HitInfo, Raycastable};

use super::MeshCollider;

#[derive(Debug)]
pub struct SphereCollider;

impl Component for SphereCollider {
    fn get_type(&self) -> TypeId {
        TypeId::of::<SphereCollider>()
    }
}

impl Raycastable for SphereCollider {
    fn ray_intersects(
        &self,
        _object_manager: &ObjectManager,
        _start: V3,
        _dir: V3,
        _max_distance: Option<f32>,
    ) -> Vec<HitInfo> {
        todo!()
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
