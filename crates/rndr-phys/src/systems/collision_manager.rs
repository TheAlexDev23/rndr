use rndr_core::object::ObjectManager;

use rndr_math::prelude::V3;

use crate::collision;

pub struct CollisionInfo {
    pub position: V3,
    pub normal: V3,
    pub obj_1: u64,
    pub obj_2: u64,
}

#[derive(Default)]
pub(crate) struct CollisionManager;

impl CollisionManager {
    pub fn calculate(&self, object_manager: &mut ObjectManager) -> Vec<CollisionInfo> {
        let mut calculated_hits = Vec::new();
        let mut all_hits = Vec::new();
        for object in object_manager.objects_iter() {
            let object_id = object.id();
            let collidable = collision::get_trait_collidable(object);
            if collidable.is_none() {
                continue;
            }

            for collision_comparator in object_manager.objects_iter() {
                let collision_comparator_id = collision_comparator.id();
                if collision_comparator_id == object_id {
                    continue;
                }
                let other = collision::get_dynamic_collidable(collision_comparator);
                if other.is_none() {
                    continue;
                }

                if calculated_hits.iter().any(|comb| {
                    *comb == (object_id, collision_comparator_id)
                        || *comb == (collision_comparator_id, object_id)
                }) {
                    continue;
                }

                if let Some(hit) = collidable
                    .unwrap()
                    .intersects_dynamic_collidable(other.unwrap(), object_manager)
                {
                    calculated_hits.push((object_id, collision_comparator_id));
                    all_hits.push(CollisionInfo {
                        position: hit.position,
                        normal: hit.normal,
                        obj_1: object_id,
                        obj_2: collision_comparator_id,
                    });
                }
            }
        }
        all_hits
    }
}
