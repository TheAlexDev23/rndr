use std::any::TypeId;

use rndr_core::object::Component;
use rndr_math::vector::V3;

static mut GRAVITY_ACCELERATION: f32 = -5.0;

#[derive(Debug, Default)]
pub struct Rigidbody {
    pub lock_movement: bool,
    pub affected_by_gravity: bool,

    pub bounciness: f32,

    pub last_velocity: V3,
    pub velocity: V3,
    pub mass: f32,

    owner: Option<u64>,
}

impl Component for Rigidbody {
    fn get_type(&self) -> std::any::TypeId {
        TypeId::of::<Rigidbody>()
    }

    fn on_added(&mut self, object: u64) {
        self.owner = Some(object);
    }
}

impl Rigidbody {
    pub fn new(mass: f32) -> Rigidbody {
        Rigidbody {
            mass,
            ..Default::default()
        }
    }
    pub fn new_with_gravity(mass: f32) -> Rigidbody {
        Rigidbody {
            mass,
            affected_by_gravity: true,
            ..Default::default()
        }
    }

    pub fn calculate_acceleration(&self, dt: f32) -> V3 {
        (self.velocity - self.last_velocity) / dt
    }

    pub fn tick(&mut self, dt: f32) -> (V3, V3) {
        let ret_pos = self.velocity * dt;

        if self.lock_movement {
            self.last_velocity = V3::default();
            self.velocity = V3::default();
            return (V3::default(), V3::default());
        }

        if self.affected_by_gravity {
            self.velocity += V3::new(0.0, 0.0, unsafe { GRAVITY_ACCELERATION } * dt);
        }

        self.last_velocity = self.velocity;
        (ret_pos, V3::default())
    }
}
