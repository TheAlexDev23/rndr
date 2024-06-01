use rndr_core::{
    default_components::{render::MeshRenderable, Transform},
    object::ObjectManager,
};

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

            let rb = object.component_mut::<Rigidbody>();
            let (pos_delta, rot_delta) = rb.tick(dt);

            let transform = object.component_mut::<Transform>();

            transform.position += pos_delta;
            transform.rotation += rot_delta;
        }

        let collisions = self.collision_manager.calculate(object_manager);

        self.react_to_collisions(collisions, object_manager, dt);
    }

    fn react_to_collisions(
        &self,
        collisions: Vec<CollisionInfo>,
        object_manager: &mut ObjectManager,
        dt: f32,
    ) {
        for collision in collisions {
            let obj1 = object_manager.get_object(collision.obj_1);
            let obj2 = object_manager.get_object(collision.obj_2);

            let rb1 = obj1.component::<Rigidbody>();
            let rb2 = obj2.component::<Rigidbody>();

            let mesh1 = obj1.component::<MeshRenderable>();
            let mesh2 = obj2.component::<MeshRenderable>();

            let tr1 = obj1.component::<Transform>();
            let tr2 = obj2.component::<Transform>();

            let collision_offset1 = collision.position - mesh1.calculate_center(tr1);
            let collision_offset2 = collision.position - mesh2.calculate_center(tr2);

            let f1 = rb1.linear_velocity / dt * rb1.mass
                - rb1.angular_velocity.cross(collision_offset1) / dt * rb1.mass;
            let f2 = rb2.linear_velocity / dt * rb2.mass
                - rb2.angular_velocity.cross(collision_offset2) / dt * rb2.mass;

            let f1 = f1.dot(collision.normal) * collision.normal;
            let f2 = f2.dot(collision.normal) * collision.normal;

            let rb1 = object_manager
                .get_object_mut(collision.obj_1)
                .component_mut::<Rigidbody>();

            if !rb1.lock_movement {
                rb1.linear_velocity += -1.0 * f1 / rb1.mass * dt + f2 / rb1.mass * dt;
            }
            if !rb1.lock_rotation {
                rb1.angular_velocity += -1.0 * f1.cross(collision_offset1) / rb1.mass * dt
                    + f2.cross(collision_offset1) / rb1.mass * dt;
            }

            let rb2 = object_manager
                .get_object_mut(collision.obj_2)
                .component_mut::<Rigidbody>();

            if !rb2.lock_movement {
                rb2.linear_velocity += -1.0 * f2 / rb2.mass * dt + f1 / rb2.mass * dt;
            }
            if !rb2.lock_rotation {
                rb2.angular_velocity += -1.0 * f2.cross(collision_offset2) / rb2.mass * dt
                    + f2.cross(collision_offset2) / rb2.mass * dt;
            }
        }
    }
}
