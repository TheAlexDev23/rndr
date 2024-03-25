use rndr_math::vector::V3;

pub struct FragData {
    pub position: V3,
    pub color: [u8; 3],
}

pub trait FragShader {
    fn frag(&self, data: &mut FragData);
}
