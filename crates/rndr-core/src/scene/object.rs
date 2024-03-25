use rndr_math::prelude::Transform;
use rndr_math::prelude::V3;

#[derive(Default, Clone, Copy)]
pub struct Vertex {
    pub position: V3,
    pub color: [u8; 3],
}

#[derive(Clone)]
pub struct Object {
    pub transform: Transform,
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<usize>,
}

impl Vertex {
    pub fn new(position: V3) -> Vertex {
        Vertex {
            position,
            color: [255, 255, 255],
        }
    }

    pub fn new_with_color(position: V3, color: [u8; 3]) -> Vertex {
        Vertex { position, color }
    }

    pub fn interpolate_color(v1: (Vertex, f32), v2: (Vertex, f32), v3: (Vertex, f32)) -> [u8; 3] {
        [
            (v1.0.color[0] as f32 * v1.1
                + v2.0.color[0] as f32 * v2.1
                + v3.0.color[0] as f32 * v3.1) as u8,
            (v1.0.color[1] as f32 * v1.1
                + v2.0.color[1] as f32 * v2.1
                + v3.0.color[1] as f32 * v3.1) as u8,
            (v1.0.color[2] as f32 * v1.1
                + v2.0.color[2] as f32 * v2.1
                + v3.0.color[2] as f32 * v3.1) as u8,
        ]
    }
}
