use std::any::TypeId;

use rndr_math::prelude::Vertex;

use crate::prelude::{Component, FragShader};
use crate::render::shader;

pub struct MeshRenderable {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<[usize; 3]>,
    pub shader: Box<dyn FragShader>,
}

impl MeshRenderable {
    pub fn from_stl(path: &str) -> Result<MeshRenderable, std::io::Error> {
        let mut file = std::fs::File::open(path)?;
        let stl = stl::read_stl(&mut file)?;
        let mut object = MeshRenderable {
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

impl Component for MeshRenderable {
    fn get_type(&self) -> TypeId {
        TypeId::of::<MeshRenderable>()
    }
}
