use std::any::TypeId;

use rndr_core::object::Component;
use rndr_math::vector::V3;

#[derive(Debug, Default)]
pub struct Rigidbody {
    owner: Option<u64>,
    pub acceleration: V3,
    pub velocity: V3,
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
    pub fn tick(&mut self, dt: f32) -> (V3, V3) {
        self.velocity += self.acceleration * dt;

        (self.velocity * dt, V3::default())
    }
}
