use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::prelude::M3x3;

#[derive(Debug, Default, Clone, Copy)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl V3 {
    pub fn new(x: f32, y: f32, z: f32) -> V3 {
        V3 { x, y, z }
    }

    pub fn average(&self, other: V3) -> V3 {
        (*self + other) / 2.0
    }

    pub fn relative_to(&self, other: &V3) -> V3 {
        *self - *other
    }

    pub fn normalize(&mut self) {
        *self = self.norm()
    }

    pub fn norm(&self) -> V3 {
        if self.mag() == 0.0 {
            V3::default()
        } else {
            *self / self.mag()
        }
    }

    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn cross(&self, other: V3) -> V3 {
        V3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn dot(&self, other: V3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn hadamard_product(&self, other: V3) -> V3 {
        V3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn all_elements_sum(&self) -> f32 {
        self.x + self.y + self.z
    }

    pub fn inverse(&self) -> V3 {
        V3::new(self.x.powi(-1), self.y.powi(-1), self.z.powi(-1))
    }

    /// Handle the V3 as a 3D point and rotate by `angle`, where angle is not a 3d point
    /// but an (x, y, z) euler rotation
    pub fn rotate(&self, angle: V3) -> V3 {
        Self::rotation_matrix(angle) * *self
    }

    pub fn rotate_in_bulk(vectors: Vec<&mut V3>, angle: V3) {
        let rotation_matrix = Self::rotation_matrix(angle);
        for vector in vectors.into_iter() {
            *vector = *vector * rotation_matrix;
        }
    }

    fn rotation_matrix(angle: V3) -> M3x3 {
        let a = angle.z;
        let b = angle.y;
        let y = angle.x;

        let sin_a = a.to_radians().sin();
        let sin_b = b.to_radians().sin();
        let sin_y = y.to_radians().sin();

        let cos_a = a.to_radians().cos();
        let cos_b = b.to_radians().cos();
        let cos_y = y.to_radians().cos();

        M3x3::new([
            V3::new(cos_a * cos_b, sin_a * cos_b, -sin_b),
            V3::new(
                cos_a * sin_b * sin_y - sin_a * cos_y,
                sin_a * sin_b * sin_y + cos_a * cos_y,
                cos_b * sin_y,
            ),
            V3::new(
                cos_a * sin_b * cos_y + sin_a * sin_y,
                sin_a * sin_b * cos_y - cos_a * sin_y,
                cos_b * cos_y,
            ),
        ])
    }

    pub fn interpolate3(v1: (V3, f32), v2: (V3, f32), v3: (V3, f32)) -> V3 {
        v1.0 * v1.1 + v2.0 * v2.1 + v3.0 * v3.1
    }

    pub fn find_reflection_vector(incident_vector: V3, surface_normal: V3) -> V3 {
        incident_vector - 2.0 * (incident_vector.dot(surface_normal) * surface_normal)
    }
}

impl Display for V3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {}, {}, {} }}", self.x, self.y, self.z)
    }
}

impl From<[f32; 3]> for V3 {
    fn from(value: [f32; 3]) -> Self {
        V3::new(value[0], value[1], value[2])
    }
}

impl Add for V3 {
    type Output = V3;
    fn add(self, rhs: Self) -> Self::Output {
        V3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for V3 {
    type Output = V3;
    fn sub(self, rhs: Self) -> Self::Output {
        V3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl AddAssign for V3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for V3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f32> for V3 {
    type Output = V3;
    fn mul(mut self, rhs: f32) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}

impl MulAssign<f32> for V3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<V3> for f32 {
    type Output = V3;
    fn mul(self, mut rhs: V3) -> Self::Output {
        rhs.x *= self;
        rhs.y *= self;
        rhs.z *= self;
        rhs
    }
}

impl Neg for V3 {
    type Output = V3;
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Div<f32> for V3 {
    type Output = V3;
    fn div(mut self, rhs: f32) -> Self::Output {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self
    }
}

impl DivAssign<f32> for V3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl PartialEq for V3 {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f32::EPSILON
            && (self.y - other.y).abs() < f32::EPSILON
            && (self.z - other.z).abs() < f32::EPSILON
    }
}
