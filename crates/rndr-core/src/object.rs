use rndr_math::prelude::Transform;
use rndr_math::prelude::V3;

#[derive(Clone)]
pub struct Object {
    pub transform: Transform,
    pub vertices: Vec<V3>,
    pub triangles: Vec<usize>,
}
