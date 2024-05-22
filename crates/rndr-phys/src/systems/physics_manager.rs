use rndr_core::{default_components::Transform, object::ObjectManager};

use crate::components::rigidbody::Rigidbody;

use super::{collision_manager::CollisionInfo, CollisionManager};

#[derive(Default)]
pub struct PhysicsManager {
    collision_manager: CollisionManager,
}

impl PhysicsManager {
    pub fn tick(&self, object_manager: &mut ObjectManager, dt: f32) {
        for object in object_manager.objects_iter_mut() {
            if !object.has_component::<Rigidbody>() {
                continue;
            }

            let rb = object.component_mut::<Rigidbody>().unwrap();
            let (pos_delta, rot_delta) = rb.tick(dt);

            let transform = object
                .component_mut::<Transform>()
                .expect("Object with rigidbody does not have transform");

            let last_position = transform.position;

            transform.position += pos_delta;
            transform.rotation += rot_delta;

            object.component_mut::<Rigidbody>().unwrap().last_position = last_position;
        }

        let collisions = self.collision_manager.calculate(object_manager);

        self.react_to_collisions(collisions, object_manager);
    }

    fn react_to_collisions(
        &self,
        collisions: Vec<CollisionInfo>,
        object_manager: &mut ObjectManager,
    ) {
        for collision in collisions {
            let rb_1 = object_manager
                .get_object(collision.obj_1)
                .unwrap()
                .component::<Rigidbody>()
                .unwrap();
            let rb_2 = object_manager
                .get_object(collision.obj_2)
                .unwrap()
                .component::<Rigidbody>()
                .unwrap();

            let momentum_1 = (1.0 - rb_1.bounciness) * rb_1.velocity * rb_1.mass;
            let momentum_2 = (1.0 - rb_2.bounciness) * rb_2.velocity * rb_2.mass;

            let rb_1_mass = rb_1.mass;
            let rb_2_mass = rb_2.mass;

            let pos_1 = rb_1.last_position;
            let pos_2 = rb_2.last_position;

            object_manager
                .get_object_mut(collision.obj_1)
                .unwrap()
                .component_mut::<Transform>()
                .unwrap()
                .position = pos_1;
            object_manager
                .get_object_mut(collision.obj_2)
                .unwrap()
                .component_mut::<Transform>()
                .unwrap()
                .position = pos_2;

            let rb_1 = object_manager
                .get_object_mut(collision.obj_1)
                .unwrap()
                .component_mut::<Rigidbody>()
                .unwrap();

            if !rb_1.lock_movement {
                rb_1.velocity = rb_1.bounciness * (-1.0 * rb_1.velocity) + momentum_2 / rb_1_mass;
                println!("Setting velocity of 1 to: {}", rb_1.velocity);
            }

            let rb_2 = object_manager
                .get_object_mut(collision.obj_2)
                .unwrap()
                .component_mut::<Rigidbody>()
                .unwrap();

            if !rb_2.lock_movement {
                rb_2.velocity = rb_2.bounciness * (-1.0 * rb_2.velocity) + momentum_1 / rb_2_mass;
            }
        }
    }
}
