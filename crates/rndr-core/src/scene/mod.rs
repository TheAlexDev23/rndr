pub mod object;

pub use object::Object;

pub(crate) struct SceneContext {
    pub objects: Vec<Object>,
}
