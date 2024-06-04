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

    pub fn inv(&self) -> Option<M3x3> {
        let a = self.columns[0].x;
        let b = self.columns[1].x;
        let c = self.columns[2].x;

        let d = self.columns[0].y;
        let e = self.columns[1].y;
        let f = self.columns[2].y;

        let g = self.columns[0].z;
        let h = self.columns[1].z;
        let i = self.columns[2].z;

        let determinant =
            (a * e * i) + (b * f * g) + (c * d * h) - (c * e * g) - (b * d * i) - (a * f * h);
        if determinant < f32::EPSILON && determinant > -f32::EPSILON {
            return None;
        }

        let mut a_ = e * i - f * h;
        let mut b_ = -(d * i - f * g);
        let mut c_ = d * h - e * g;

        let mut d_ = -(b * i - c * h);
        let mut e_ = a * i - c * g;
        let mut f_ = -(a * h - b * g);

        let mut g_ = b * f - c * e;
        let mut h_ = -(a * f - c * d);
        let mut i_ = a * e - b * d;

        let mult = 1.0 / determinant;

        a_ *= mult;
        b_ *= mult;
        c_ *= mult;
        d_ *= mult;
        e_ *= mult;
        f_ *= mult;
        g_ *= mult;
        h_ *= mult;
        i_ *= mult;

        Some(M3x3::new([
            V3::new(a_, b_, c_),
            V3::new(d_, e_, f_),
            V3::new(g_, h_, i_),
        ]))
    }
}

impl Mul<V3> for M3x3 {
    type Output = V3;
    fn mul(self, rhs: V3) -> Self::Output {
        self.columns[0] * rhs.x + self.columns[1] * rhs.y + self.columns[2] * rhs.z
    }
}

impl Mul<M3x3> for V3 {
    type Output = V3;
    fn mul(self, rhs: M3x3) -> Self::Output {
        rhs * self
    }
}
