use std::any::TypeId;
use std::fmt::Debug;

use rndr_math::prelude::{Vertex, V3};

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
        let mut ret = MeshRenderable {
            vertices: vec![
                Vertex::new_with_color(V3::new(-1.0, 0.0, -1.0), [255; 3]),
                Vertex::new_with_color(V3::new(-1.0, 0.0, 1.0), [255; 3]),
                Vertex::new_with_color(V3::new(1.0, 0.0, 1.0), [255; 3]),
                Vertex::new_with_color(V3::new(1.0, 0.0, -1.0), [255; 3]),
            ],
            vertices_center: V3::default(),
            triangles: vec![[0, 1, 2], [0, 2, 3]],
            shader: Box::from(DefaultShader),
        };
        ret.vertices_center = Self::find_vertex_average(&ret.vertices);
        ret
    }

    pub fn from_stl(path: &str) -> Result<MeshRenderable, std::io::Error> {
        let mut file = std::fs::File::open(path)?;
        let stl = stl::read_stl(&mut file)?;
        let mut object = MeshRenderable {
            vertices: Vec::with_capacity(stl.triangles.len() * 3),
            triangles: Vec::with_capacity(stl.triangles.len()),
            vertices_center: V3::default(),
            shader: Box::new(DefaultShader),
        };
        for triangle in stl.triangles {
            let len = object.vertices.len();

            object.vertices.push(Vertex {
                color: [255, 0, 0],
                position: triangle.v1.into(),
                ..Default::default()
            });
            object.vertices.push(Vertex {
                color: [0, 255, 0],
                position: triangle.v2.into(),
                ..Default::default()
            });
            object.vertices.push(Vertex {
                color: [0, 0, 255],
                position: triangle.v3.into(),
                ..Default::default()
            });

            object.triangles.push([len, len + 1, len + 2]);
        }

        object.vertices_center = Self::find_vertex_average(&object.vertices);

        for triangle in object.triangles.iter() {
            Self::set_triangle_vertices_normal(triangle, &mut object.vertices);
        }

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

    fn set_triangle_vertices_normal(triangle: &[usize; 3], all_vertices: &mut [Vertex]) {
        let triangle_side_1 =
            all_vertices[triangle[0]].position - all_vertices[triangle[1]].position;
        let triangle_side_2 =
            all_vertices[triangle[1]].position - all_vertices[triangle[2]].position;
        let triangle_normal = triangle_side_1.cross(triangle_side_2).norm();

        for vertex in triangle {
            all_vertices[*vertex].normal = triangle_normal;
        }
    }
}

impl Component for MeshRenderable {
    fn get_type(&self) -> TypeId {
        TypeId::of::<MeshRenderable>()
    }
}
