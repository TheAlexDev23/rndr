use getset::{Getters, MutGetters, Setters};
use rndr_math::vector::V3;

#[derive(Getters, MutGetters, Setters)]
pub struct FragData {
    #[getset(get)]
    pub space_position: V3,
    #[getset(get)]
    pub relative_position: V3,

    #[getset(get, set, get_mut)]
    pub vertex_color: [u8; 3],
}

pub trait FragShader {
    fn frag(&self, data: &mut FragData);
}
