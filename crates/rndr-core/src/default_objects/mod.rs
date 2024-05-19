use rndr_math::prelude::{Vertex, V3};

use crate::default_components::render::{Camera, MeshRenderable};
use crate::default_components::Transform;
use crate::object::Object;
use crate::render::shader::DefaultShader;

pub fn camera(perspective: bool) -> Object {
    let mut camera = Object::new();
    camera.add_component(Box::new(Transform::default()));
    camera.add_component(Box::new(Camera::new(perspective)));
    camera
}

pub fn stl_mesh(path: &str) -> Result<Object, std::io::Error> {
    let mut object = Object::new();
    object.add_component(Box::new(MeshRenderable::from_stl(path)?));
    object.add_component(Box::new(Transform::default()));
    Ok(object)
}

pub fn plane() -> Object {
    let mut object = Object::new();
    object.add_component(
        MeshRenderable {
            vertices: vec![
                Vertex::new_with_color(V3::new(-1.0, 0.0, -1.0), [255; 3]),
                Vertex::new_with_color(V3::new(-1.0, 0.0, 1.0), [255; 3]),
                Vertex::new_with_color(V3::new(1.0, 0.0, 1.0), [255; 3]),
                Vertex::new_with_color(V3::new(1.0, 0.0, -1.0), [255; 3]),
            ],
            triangles: vec![[0, 1, 2], [0, 2, 3]],
            shader: Box::from(DefaultShader),
        }
        .into(),
    );
    object.add_component(Box::new(Transform::default()));

    object
}
