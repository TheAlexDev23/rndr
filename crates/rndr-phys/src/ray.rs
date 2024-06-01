use rndr_core::object::{Object, ObjectManager};
use rndr_math::vector::V3;

use crate::traits::raycastable::get_raycastable;
use crate::traits::HitInfo;

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

            intersects.extend(raycastable.get_all_ray_intersections(
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
        raycastable.get_all_ray_intersections(
            object_manager,
            self.start,
            self.dir,
            self.max_distance,
        )
    }
}
