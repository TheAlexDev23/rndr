use crate::default_components::render::{Camera, MeshRenderable};
use crate::default_components::Transform;
use crate::object::Object;

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
