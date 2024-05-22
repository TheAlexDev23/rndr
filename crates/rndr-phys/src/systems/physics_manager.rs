use rndr_core::{default_components::Transform, object::ObjectManager};

use crate::components::rigidbody::Rigidbody;

pub struct PhysicsManager;

impl PhysicsManager {
    pub fn tick(&self, object_manager: &mut ObjectManager, dt: f32) {
        for object in object_manager.objects_iter_mut() {
            if let Some(rb) = object.component_mut::<Rigidbody>() {
                let (pos_delta, rot_delta) = rb.tick(dt);
                let transform = object
                    .component_mut::<Transform>()
                    .expect("Object with rigidbody does not have transform");
                transform.position += pos_delta;
                transform.rotation += rot_delta;
            }
        }
    }
}
