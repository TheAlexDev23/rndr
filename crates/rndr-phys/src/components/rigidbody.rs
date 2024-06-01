use std::any::TypeId;

use rndr_core::object::Component;
use rndr_math::vector::V3;

static mut GRAVITY_ACCELERATION: f32 = -5.0;

#[derive(Debug, Default)]
pub struct Rigidbody {
    pub lock_movement: bool,
    pub lock_rotation: bool,
    pub affected_by_gravity: bool,

    pub mass: f32,

    pub linear_velocity: V3,
    pub angular_velocity: V3,

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

    pub fn tick(&mut self, dt: f32) -> (V3, V3) {
        let start_lv = self.linear_velocity;
        let start_av = self.angular_velocity;
        if self.affected_by_gravity {
            self.linear_velocity += V3::new(0.0, 0.0, unsafe { GRAVITY_ACCELERATION } * dt);
        }

        if self.lock_movement {
            self.linear_velocity = V3::default();
        }

        if self.lock_rotation {
            self.angular_velocity = V3::default();
        }

        (start_lv * dt, start_av * dt)
    }
}
