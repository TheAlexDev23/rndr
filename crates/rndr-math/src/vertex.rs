use crate::prelude::V3;

#[derive(Default, Debug, Clone, Copy)]
pub struct Vertex {
    pub position: V3,
    pub normal: V3,
    pub color: [u8; 3],
}

impl Vertex {
    pub fn new(position: V3, color: [u8; 3], normal: V3) -> Vertex {
        Vertex {
            position,
            color,
            normal,
        }
    }

    pub fn new_with_position(position: V3) -> Vertex {
        Vertex {
            position,
            color: [255, 255, 255],
            ..Default::default()
        }
    }

    pub fn new_with_color(position: V3, color: [u8; 3]) -> Vertex {
        Vertex {
            position,
            color,
            ..Default::default()
        }
    }

    pub fn interpolate(v1: (Vertex, f32), v2: (Vertex, f32), v3: (Vertex, f32)) -> Vertex {
        let color = Self::interpolate_color(v1, v2, v3);
        let n1 = (v1.0.normal, v1.1);
        let n2 = (v2.0.normal, v2.1);
        let n3 = (v3.0.normal, v3.1);

        let v1 = (v1.0.position, v1.1);
        let v2 = (v2.0.position, v2.1);
        let v3 = (v3.0.position, v3.1);

        Vertex {
            color,
            position: V3::interpolate3(v1, v2, v3),
            normal: V3::interpolate3(n1, n2, n3),
            ..Default::default()
        }
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
