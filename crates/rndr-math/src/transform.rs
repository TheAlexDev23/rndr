use super::prelude::V3;

#[derive(Default, Clone)]
pub struct Transform {
    pub position: V3,
    pub rotation: V3,
}

const FWD: V3 = V3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};
const RIGHT: V3 = V3 {
    x: 0.0,
    y: -1.0,
    z: 0.0,
};

const UP: V3 = V3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

impl Transform {
    pub fn get_orientations_in_bulk(&self) -> (V3, V3, V3) {
        let (mut fwd, mut right, mut up) = (FWD, RIGHT, UP);
        V3::rotate_in_bulk(vec![&mut fwd, &mut right, &mut up], self.rotation);
        (fwd, right, up)
    }

    pub fn fwd(&self) -> V3 {
        FWD.rotate(self.rotation)
    }

    pub fn right(&self) -> V3 {
        RIGHT.rotate(self.rotation)
    }

    pub fn up(&self) -> V3 {
        UP.rotate(self.rotation)
    }
}
