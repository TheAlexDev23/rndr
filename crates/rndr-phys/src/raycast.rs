use rndr_core::object::{Object, ObjectManager};

use rndr_math::prelude::{Vertex, V3};

use crate::components::{MeshCollider, SphereCollider};

#[derive(Debug)]
pub struct HitInfo {
    pub vertex: Vertex,
    pub distance: f32,
}

/// Represents an object intersectable by a ray
pub trait Raycastable {
    fn ray_intersects(
        &self,
        object_manager: &ObjectManager,
        start: V3,
        dir: V3,
        max_distance: Option<f32>,
    ) -> Vec<HitInfo>;
}

fn get_raycastable(object: &Object) -> Option<&dyn Raycastable> {
    if let Some(r) = object.component::<MeshCollider>() {
        return Some(r);
    }
    if let Some(r) = object.component::<SphereCollider>() {
        return Some(r);
    }
    None
}

pub struct Ray<'a> {
    pub start: V3,
    pub dir: V3,
    pub max_distance: Option<f32>,

    pub objects: &'a ObjectManager,
}

impl<'a> Ray<'a> {
    pub fn cast(&self) -> Option<HitInfo> {
        let mut intersects = Vec::new();
        for obj in self.objects.objects_iter() {
            let raycastable = match get_raycastable(obj) {
                Some(raycastable) => raycastable,
                None => continue,
            };

            intersects.extend(raycastable.ray_intersects(
                &self.objects,
                self.start,
                self.dir,
                self.max_distance,
            ));
        }

        intersects
            .into_iter()
            .reduce(|a, b| if a.distance < b.distance { a } else { b })
    }
}

pub struct ObjectIntersectionRay<'a> {
    pub start: V3,
    pub dir: V3,
    pub max_distance: Option<f32>,

    pub object: &'a Object,
}

impl<'a> ObjectIntersectionRay<'a> {
    pub fn cast(&self, object_manager: &ObjectManager) -> Vec<HitInfo> {
        let raycastable = get_raycastable(&self.object).unwrap();
        raycastable.ray_intersects(object_manager, self.start, self.dir, self.max_distance)
    }
}
