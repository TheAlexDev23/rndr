use std::fmt::Debug;

use getset::{Getters, MutGetters, Setters};

use rndr_math::vector::V3;

#[derive(Getters, MutGetters, Setters)]
pub struct FragData {
    #[getset(get = "pub")]
    pub(crate) space_position: V3,
    #[getset(get = "pub")]
    pub(crate) relative_position: V3,

    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub(crate) output_pixel: (f32, [u8; 3]),
}

pub trait FragShader: Debug + Sync + Send {
    fn frag(&self, data: &mut FragData);
}

#[derive(Debug)]
pub struct DefaultShader;
impl FragShader for DefaultShader {
    fn frag(&self, _data: &mut FragData) {}
}
