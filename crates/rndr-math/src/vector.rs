use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

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

    pub fn relative_to(&self, other: &V3) -> V3 {
        *self - *other
    }
}

impl Display for V3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {}, {}, {} }}", self.x, self.y, self.z)
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

impl Mul<f32> for V3 {
    type Output = V3;
    fn mul(mut self, rhs: f32) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}
