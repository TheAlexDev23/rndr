use super::prelude::V3;

#[derive(Default, Clone)]
pub struct Transform {
    pub position: V3,
    pub rotation: V3,
}

impl Transform {
    pub fn fwd(&self) -> V3 {
        let mut fwd = V3::new(1.0, 0.0, 0.0);
        fwd.rotate(self.rotation);
        fwd
    }

    pub fn right(&self) -> V3 {
        let mut right = V3::new(0.0, -1.0, 0.0);
        right.rotate(self.rotation);
        right
    }

    pub fn up(&self) -> V3 {
        let mut up = V3::new(0.0, 0.0, 1.0);
        up.rotate(self.rotation);
        up
    }
}
