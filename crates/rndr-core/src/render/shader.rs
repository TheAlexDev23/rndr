use rndr_math::vector::V3;

use crate::prelude::object::Vertex;

pub struct FragData {
    pub vertex: Vertex,
}

pub trait FragShader {
    fn frag(&self, data: &mut FragData);
}
