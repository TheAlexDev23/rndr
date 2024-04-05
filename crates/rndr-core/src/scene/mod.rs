pub mod object;

pub use object::Object;

pub struct SceneContext {
    pub objects: Vec<Object>,
}
