use std::any::TypeId;
use std::fmt::Debug;

use rndr_math::prelude::{Vertex, V3};
use russimp::scene::{PostProcess, Scene};
use russimp::RussimpError;

use crate::default_components::Transform;
use crate::object::Component;
use crate::render::{shader::DefaultShader, FragShader};

#[derive(Debug)]
pub struct MeshRenderable {
    pub vertices: Vec<Vertex>,
    vertices_center: V3,
    pub triangles: Vec<[usize; 3]>,
    pub shader: Box<dyn FragShader>,
}

impl MeshRenderable {
    pub fn plane() -> MeshRenderable {
        let n = V3::new(0.0, 1.0, 0.0);
        let mut ret = MeshRenderable {
            vertices: vec![
                Vertex::new(V3::new(-1.0, 0.0, -1.0), [255; 3], n),
                Vertex::new(V3::new(-1.0, 0.0, 1.0), [255; 3], n),
                Vertex::new(V3::new(1.0, 0.0, 1.0), [255; 3], n),
                Vertex::new(V3::new(1.0, 0.0, -1.0), [255; 3], n),
            ],
            vertices_center: V3::default(),
            triangles: vec![[0, 1, 2], [0, 2, 3]],
            shader: Box::from(DefaultShader),
        };
        ret.vertices_center = Self::find_vertex_average(&ret.vertices);
        ret
    }
    pub fn small_plane() -> MeshRenderable {
        let mut ret = MeshRenderable {
            vertices: vec![
                Vertex::new_with_color(V3::new(-0.2, 0.0, -0.2), [255; 3]),
                Vertex::new_with_color(V3::new(-0.2, 0.0, 0.2), [255; 3]),
                Vertex::new_with_color(V3::new(0.2, 0.0, 0.2), [255; 3]),
                Vertex::new_with_color(V3::new(0.2, 0.0, -0.2), [255; 3]),
            ],
            vertices_center: V3::default(),
            triangles: vec![[0, 1, 2], [0, 2, 3]],
            shader: Box::from(DefaultShader),
        };
        ret.vertices_center = Self::find_vertex_average(&ret.vertices);
        ret
    }

    pub fn from_file(path: &str) -> Result<MeshRenderable, RussimpError> {
        let mesh = &Scene::from_file(
            path,
            vec![
                PostProcess::JoinIdenticalVertices,
                PostProcess::GenerateNormals,
                PostProcess::Triangulate,
            ],
        )?
        .meshes[0];

        let mut object = MeshRenderable {
            vertices: Vec::default(),
            triangles: Vec::default(),
            vertices_center: V3::default(),
            shader: Box::new(DefaultShader),
        };

        // In meshes that reuse vertices for multiple faces we need to check if some vertices haven't been pushed in yet
        let mut pushed_vertices = Vec::new();

        for face in mesh.faces.iter() {
            object
                .triangles
                .push([face.0[0] as usize, face.0[1] as usize, face.0[2] as usize]);

            for i in 0..face.0.len() {
                let idx = face.0[i] as usize;
                if pushed_vertices.iter().find(|v| **v == idx).is_some() {
                    continue;
                }
                pushed_vertices.push(idx);
                let p = mesh.vertices[idx];
                let p = V3::new(p.x, p.y, p.z);
                let n = mesh.normals[idx];
                let n = V3::new(n.x, n.y, n.z);
                let v = Vertex {
                    position: V3::new(p.x, p.y, p.z),
                    normal: V3::new(n.x, n.y, n.z),
                    color: if i == 0 {
                        [255, 0, 0]
                    } else if i == 1 {
                        [0, 255, 0]
                    } else {
                        [0, 0, 255]
                    },
                };
                object.vertices.push(v);
            }
        }

        object.vertices_center = Self::find_vertex_average(&object.vertices);

        Ok(object)
    }

    pub fn calculate_center(&self, transform: &Transform) -> V3 {
        let mut center = self.vertices_center;
        center += transform.position;
        center
    }

    fn find_vertex_average(vertices: &[Vertex]) -> V3 {
        let mut center_x = 0.0;
        let mut center_y = 0.0;
        let mut center_z = 0.0;

        for vertex in vertices.iter().cloned() {
            center_x += vertex.position.x;
            center_y += vertex.position.y;
            center_z += vertex.position.z;
        }

        let vertices_len = vertices.len() as f32;

        center_x /= vertices_len;
        center_y /= vertices_len;
        center_z /= vertices_len;

        V3::new(center_x, center_y, center_z)
    }
}

impl Component for MeshRenderable {
    fn get_type(&self) -> TypeId {
        TypeId::of::<MeshRenderable>()
    }
}
