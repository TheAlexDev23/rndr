use std::ops::Mul;

use crate::prelude::V3;

#[derive(Clone, Copy, Debug)]
pub struct M3x3 {
    pub columns: [V3; 3],
}

impl M3x3 {
    pub fn new(columns: [V3; 3]) -> M3x3 {
        M3x3 { columns }
    }
}

impl Mul<V3> for M3x3 {
    type Output = V3;
    fn mul(self, rhs: V3) -> Self::Output {
        self.columns[0] * rhs.x + self.columns[1] * rhs.y + self.columns[2] * rhs.z
    }
}
