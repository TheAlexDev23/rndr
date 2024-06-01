use rndr_core::object::{Object, ObjectManager};

use rndr_math::prelude::V3;

use crate::components::{MeshCollider, SphereCollider};

pub struct IntersectionPoint {
    pub position: V3,
    /// Collision normal. Exclusively from object 1 to object 2.
    pub normal: V3,
}

/// Represents an object that can intersect with another
pub trait Collidable {
    fn intersects_dynamic_collidable(
        &self,
        collidable: DynamicCollidable,
        object_manager: &ObjectManager,
    ) -> Option<IntersectionPoint> {
        match collidable {
            DynamicCollidable::Mesh(collider) => self.intersects_mesh(collider, object_manager),
            DynamicCollidable::Sphere(collider) => self.intersects_sphere(collider, object_manager),
        }
    }
    fn intersects_mesh(
        &self,
        other: &MeshCollider,
        object_manager: &ObjectManager,
    ) -> Option<IntersectionPoint>;
    fn intersects_sphere(
        &self,
        other: &SphereCollider,
        object_manager: &ObjectManager,
    ) -> Option<IntersectionPoint>;
}

pub enum DynamicCollidable<'a> {
    Mesh(&'a MeshCollider),
    Sphere(&'a SphereCollider),
}

pub fn get_trait_collidable(object: &Object) -> Option<&dyn Collidable> {
    if let Some(r) = object.try_component::<MeshCollider>() {
        return Some(r);
    }
    if let Some(r) = object.try_component::<SphereCollider>() {
        return Some(r);
    }
    None
}

pub fn get_dynamic_collidable(object: &Object) -> Option<DynamicCollidable> {
    if let Some(r) = object.try_component::<MeshCollider>() {
        return Some(DynamicCollidable::Mesh(r));
    }
    if let Some(r) = object.try_component::<SphereCollider>() {
        return Some(DynamicCollidable::Sphere(r));
    }
    None
}
