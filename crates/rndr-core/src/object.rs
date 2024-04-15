use std::collections::HashMap;

use std::any::TypeId;

use downcast_rs::{impl_downcast, Downcast};

#[derive(Default)]
pub struct Object {
    id: u64,
    components: Vec<Box<dyn Component>>,
}

impl Object {
    pub fn new() -> Object {
        Object::default()
    }

    pub(crate) fn receive_id(&mut self, id: u64) {
        self.id = id
    }

    pub fn component(&self, type_id: TypeId) -> Option<&dyn Component> {
        self.components
            .iter()
            .find(|component| component.get_type() == type_id)
            .map(|c| c.as_ref())
    }
    pub fn component_mut(&mut self, type_id: TypeId) -> Option<&mut dyn Component> {
        self.components
            .iter_mut()
            .find(|component| component.get_type() == type_id)
            .map(|c| c.as_mut())
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        let idx = self.components.len();
        self.components.push(component);
        self.components[idx].on_added(self.id)
    }
}

pub trait Component: Downcast {
    fn get_type(&self) -> TypeId;

    /// Gets called when added as component on an object
    fn on_added(&mut self, _object: u64) {}

    fn on_start(&mut self) {}
    fn on_update(&mut self) {}
    fn on_kill(&mut self) {}
}

impl_downcast!(Component);

pub struct ObjectManager {
    obj_index: u64,
    objects: HashMap<u64, Object>,
}

impl ObjectManager {
    pub fn new() -> ObjectManager {
        ObjectManager {
            obj_index: 0,
            objects: HashMap::new(),
        }
    }

    pub fn register_object(&mut self, mut object: Object) -> u64 {
        let idx = self.obj_index;
        object.receive_id(idx);
        self.objects.insert(self.obj_index, object);
        self.obj_index += 1;
        idx
    }

    pub fn get_object(&self, index: u64) -> Option<&Object> {
        self.objects.get(&index)
    }

    pub fn get_object_mut(&mut self, index: u64) -> Option<&mut Object> {
        self.objects.get_mut(&index)
    }

    pub fn objects_iter(&self) -> impl Iterator<Item = &Object> {
        self.objects.values()
    }

    pub fn objects_iter_mut(&mut self) -> impl Iterator<Item = &mut Object> {
        self.objects.values_mut()
    }
}
