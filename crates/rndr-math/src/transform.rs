use super::prelude::V3;

#[derive(Default, Clone)]
pub struct Transform {
    pub position: V3,
    pub rotation: V3,
}

impl Transform {
    pub fn fwd(&self) -> V3 {
        V3::new(1.0, 0.0, 0.0).rotate(self.rotation)
    }

    pub fn right(&self) -> V3 {
        V3::new(0.0, -1.0, 0.0).rotate(self.rotation)
    }

    pub fn up(&self) -> V3 {
        V3::new(0.0, 0.0, 1.0).rotate(self.rotation)
    }
}
