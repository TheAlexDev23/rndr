use rndr_core::object::{Object, ObjectManager};

use rndr_math::prelude::Vertex;

use crate::components::{MeshCollider, SphereCollider};

/// Represents an object that can intersect with another
pub trait Collidable {
    fn intersects_dynamic_collidable(
        &self,
        collidable: DynamicCollidable,
        object_manager: &ObjectManager,
    ) -> Option<Vertex> {
        match collidable {
            DynamicCollidable::Mesh(collider) => self.intersects_mesh(collider, object_manager),
            DynamicCollidable::Sphere(collider) => self.intersects_sphere(collider, object_manager),
        }
    }
    fn intersects_mesh(
        &self,
        other: &MeshCollider,
        object_manager: &ObjectManager,
    ) -> Option<Vertex>;
    fn intersects_sphere(
        &self,
        other: &SphereCollider,
        object_manager: &ObjectManager,
    ) -> Option<Vertex>;
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
