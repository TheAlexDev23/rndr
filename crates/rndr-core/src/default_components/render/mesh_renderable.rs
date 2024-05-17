use std::any::TypeId;
use std::fmt::Debug;

use rndr_math::prelude::{Vertex, V3};

use crate::prelude::{Component, FragShader};
use crate::render::shader;

#[derive(Debug)]
pub struct MeshRenderable {
    pub vertices: Vec<Vertex>,
    pub center: V3,
    pub triangles: Vec<[usize; 3]>,
    pub shader: Box<dyn FragShader>,
}

impl MeshRenderable {
    pub fn from_stl(path: &str) -> Result<MeshRenderable, std::io::Error> {
        let mut file = std::fs::File::open(path)?;
        let stl = stl::read_stl(&mut file)?;
        let mut object = MeshRenderable {
            center: V3::default(),
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

        let mut center_x = 0.0;
        let mut center_y = 0.0;
        let mut center_z = 0.0;

        for vertex in object.vertices.iter() {
            center_x += vertex.position.x;
            center_y += vertex.position.y;
            center_z += vertex.position.z;
        }

        center_x /= object.vertices.len() as f32;
        center_y /= object.vertices.len() as f32;
        center_z /= object.vertices.len() as f32;

        object.center = V3::new(center_x, center_y, center_z);

        Ok(object)
    }
}

impl Component for MeshRenderable {
    fn get_type(&self) -> TypeId {
        TypeId::of::<MeshRenderable>()
    }
}