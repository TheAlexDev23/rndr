use russimp::RussimpError;

use crate::default_components::render::{Camera, MeshRenderable};
use crate::default_components::Transform;
use crate::object::Object;

pub fn camera(perspective: bool) -> Object {
    let mut camera = Object::new();
    camera.add_component(Box::new(Transform::default()));
    camera.add_component(Box::new(Camera::new(perspective)));
    camera
}

pub fn mesh_from_file(path: &str) -> Result<Object, RussimpError> {
    let mut object = Object::new();
    object.add_component(Box::new(MeshRenderable::from_file(path)?));
    object.add_component(Box::new(Transform::default()));
    Ok(object)
}

pub fn plane() -> Object {
    let mut object = Object::new();
    object.add_component(MeshRenderable::plane().into());
    object.add_component(Box::new(Transform::default()));

    object
}

pub fn small_plane() -> Object {
    let mut object = Object::new();
    object.add_component(MeshRenderable::small_plane().into());
    object.add_component(Box::new(Transform::default()));

    object
}
