use rndr_core::object::{Object, ObjectManager};

use rndr_math::prelude::V3;

use crate::components::{MeshCollider, SphereCollider};

#[derive(Debug)]
pub struct HitInfo {
    pub position: V3,
    /// The normal of the surface hit at the point where the ray hit
    pub normal: V3,
    pub distance: f32,
}

/// Represents an object intersectable by a ray
pub trait Raycastable {
    fn get_all_ray_intersections(
        &self,
        object_manager: &ObjectManager,
        start: V3,
        dir: V3,
        max_distance: Option<f32>,
    ) -> Vec<HitInfo>;
}

pub fn get_raycastable(object: &Object) -> Option<&dyn Raycastable> {
    if let Some(r) = object.try_component::<MeshCollider>() {
        return Some(r);
    }
    if let Some(r) = object.try_component::<SphereCollider>() {
        return Some(r);
    }
    None
}
