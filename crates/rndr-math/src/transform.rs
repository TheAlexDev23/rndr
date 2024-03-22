use super::prelude::V3;

pub struct Transform {
    pub position: V3,
    pub rotation: V3,
}

impl Transform {
    pub fn fwd(&self) -> V3 {
        let (_, cos_y, cos_z, _, sin_y, sin_z) = self.trigs();

        V3 {
            x: cos_z * cos_y,
            y: -1f32 * sin_z * cos_y,
            z: sin_y,
        }
    }

    pub fn right(&self) -> V3 {
        let (cos_x, cos_y, cos_z, sin_x, sin_y, sin_z) = self.trigs();

        V3 {
            x: -1f32 * cos_z * sin_y * sin_x + sin_z * cos_x,
            y: -1f32 * sin_z * sin_y * sin_x - cos_z * cos_x,
            z: -1f32 * cos_y * sin_x,
        }
    }

    pub fn up(&self) -> V3 {
        let (cos_x, cos_y, cos_z, sin_x, sin_y, sin_z) = self.trigs();

        V3 {
            x: cos_z * sin_y * cos_x + sin_z * sin_x,
            y: sin_z * sin_y * cos_y - cos_z * sin_x,
            z: -1f32 * cos_y * cos_x,
        }
    }

    fn trigs(&self) -> (f32, f32, f32, f32, f32, f32) {
        let cos_x = self.rotation.x.to_radians().cos();
        let cos_y = self.rotation.y.to_radians().cos();
        let cos_z = self.rotation.z.to_radians().cos();

        let sin_x = self.rotation.x.to_radians().sin();
        let sin_y = self.rotation.y.to_radians().sin();
        let sin_z = self.rotation.z.to_radians().sin();

        (cos_x, cos_y, cos_z, sin_x, sin_y, sin_z)
    }
}
