use crate::prelude::object::Vertex;

pub struct FragData {
    pub vertex: Vertex,
}

pub trait FragShader {
    fn frag(&self, data: &mut FragData);
}
