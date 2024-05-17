use rndr_core::default_components::Transform;
use rndr_core::object::ObjectManager;

use crate::collision;

pub struct CollisionManager;

impl CollisionManager {
    pub fn tick(&self, object_manager: &mut ObjectManager) {
        let mut all_hits = Vec::new();
        for object in object_manager.objects_iter() {
            let object_id = object.id();
            let collidable = collision::get_trait_collidable(object);
            if collidable.is_none() {
                continue;
            }

            for collision_comparator in object_manager.objects_iter() {
                if collision_comparator.id() == object_id {
                    continue;
                }
                let other = collision::get_dynamic_collidable(collision_comparator);
                if other.is_none() {
                    continue;
                }

                if let Some(hit) = collidable
                    .unwrap()
                    .intersects_dynamic_collidable(other.unwrap(), object_manager)
                {
                    all_hits.push(hit);
                }
            }
        }
        for hit in all_hits {
            let mut plane = rndr_core::default_objects::plane();
            plane.component_mut::<Transform>().unwrap().position = hit.position;
            println!("{}", hit.position);
            object_manager.register_object(plane);
        }
    }
}
