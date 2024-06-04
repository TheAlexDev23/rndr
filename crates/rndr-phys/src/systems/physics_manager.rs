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

        self.react_to_collisions(collisions, object_manager);
    }

    fn react_to_collisions(
        &self,
        collisions: Vec<CollisionInfo>,
        object_manager: &mut ObjectManager,
    ) {
        for collision in collisions {
            let obj1 = object_manager.get_object(collision.obj_1);
            let obj2 = object_manager.get_object(collision.obj_2);

            let rb1 = obj1.component::<Rigidbody>();
            let rb2 = obj2.component::<Rigidbody>();

            // TODO: some might not have mesh renderable, add function to collidable trait for the center

            let mesh1 = obj1.component::<MeshRenderable>();
            let mesh2 = obj2.component::<MeshRenderable>();

            let tr1 = obj1.component::<Transform>();
            let tr2 = obj2.component::<Transform>();

            let collision_offset1 = collision.position - mesh1.calculate_center(tr1);
            let collision_offset2 = collision.position - mesh2.calculate_center(tr2);

            let p1 =
                (rb1.linear_velocity + rb1.angular_velocity.cross(collision_offset1)) * rb1.mass;
            let p2 =
                (rb2.linear_velocity + rb2.angular_velocity.cross(collision_offset2)) * rb2.mass;

            let n1 = p1.dot(collision.normal) * collision.normal;
            let n2 = p2.dot(collision.normal) * collision.normal;

            let n_total = n1 - n2;

            let tang1 = p1 - n1;
            let tang2 = p2 - n2;

            let f1s = n_total.mag() * rb1.static_friction;
            let f2s = n_total.mag() * rb2.static_friction;

            let f1d = n_total.mag() * rb1.dynamic_friction;
            let f2d = n_total.mag() * rb2.dynamic_friction;

            let f1 = if tang1.mag() <= f1s {
                -tang1
            } else {
                -tang1.norm() * f1d
            };
            let f2 = if tang2.mag() <= f2s {
                -tang2
            } else {
                -tang2.norm() * f2d
            };

            let obj1 = object_manager.get_object_mut(collision.obj_1);
            let rb1 = obj1.component_mut::<Rigidbody>();

            let impulse1 = -n1 + n2 + f1 - f2;
            let impulse2 = -n2 + n1 + f2 - f1;

            if !rb1.lock_movement {
                rb1.linear_velocity += impulse1 / rb1.mass;
            }
            if !rb1.lock_rotation {
                rb1.angular_velocity += collision_offset1
                    .cross(impulse1)
                    .hadamard_product(rb1.inertia_tensor.inverse());
            }

            let obj2 = object_manager.get_object_mut(collision.obj_2);
            let rb2 = obj2.component_mut::<Rigidbody>();

            if !rb2.lock_movement {
                rb2.linear_velocity += impulse2 / rb2.mass;
            }

            if !rb2.lock_rotation {
                rb2.angular_velocity += collision_offset2
                    .cross(impulse2)
                    .hadamard_product(rb2.inertia_tensor.inverse());
            }
        }
    }
}
