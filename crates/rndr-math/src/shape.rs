use super::prelude::Transform;
use super::prelude::V3;

pub struct Shape {
    pub transform: Transform,
    pub vertices: Vec<V3>,
    pub triangles: Vec<usize>,
}
