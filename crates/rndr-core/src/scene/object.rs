use rndr_math::prelude::Transform;
use rndr_math::prelude::V3;

use crate::prelude::shader;
use crate::prelude::FragShader;

#[derive(Default, Clone, Copy)]
pub struct Vertex {
    pub position: V3,
    pub color: [u8; 3],
}

pub struct Object {
    pub transform: Transform,
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<[usize; 3]>,
    pub shader: Box<dyn FragShader>,
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

    pub fn interpolate(v1: (Vertex, f32), v2: (Vertex, f32), v3: (Vertex, f32)) -> Vertex {
        let color = Self::interpolate_color(v1, v2, v3);
        let v1 = (v1.0.position, v1.1);
        let v2 = (v2.0.position, v2.1);
        let v3 = (v3.0.position, v3.1);

        Vertex {
            color,
            position: V3::interpolate3(v1, v2, v3),
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

impl Object {
    pub fn from_stl(path: &str) -> Result<Object, std::io::Error> {
        let mut file = std::fs::File::open(path)?;
        let stl = stl::read_stl(&mut file)?;
        let mut object = Object {
            transform: Transform::default(),
            vertices: Vec::with_capacity(stl.triangles.len() * 3),
            triangles: Vec::with_capacity(stl.triangles.len()),
            shader: Box::new(shader::DefaultShader),
        };
        for triangle in stl.triangles {
            let len = object.vertices.len();

            object.vertices.push(Vertex {
                color: [255, 0, 0],
                position: triangle.v1.into(),
            });
            object.vertices.push(Vertex {
                color: [0, 255, 0],
                position: triangle.v2.into(),
            });
            object.vertices.push(Vertex {
                color: [0, 0, 255],
                position: triangle.v3.into(),
            });

            object.triangles.push([len, len + 1, len + 2]);
        }

        Ok(object)
    }
}
