use std::{collections::HashMap, fmt::Debug};

use std::any::TypeId;

use downcast_rs::{impl_downcast, Downcast};

#[derive(Default, Debug)]
pub struct Object {
    id: u64,
    components: HashMap<TypeId, Box<dyn Component>>,
}

impl Object {
    pub fn new() -> Object {
        Object::default()
    }

    pub(crate) fn receive_id(&mut self, id: u64) {
        self.id = id
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn has_component<T: Component>(&self) -> bool {
        self.components.contains_key(&TypeId::of::<T>())
    }

    pub fn component<T: Component>(&self) -> &T {
        self.components
            .get(&TypeId::of::<T>())
            .map(|c| c.as_ref())
            .unwrap()
            .downcast_ref::<T>()
            .unwrap()
    }
    pub fn component_mut<T: Component>(&mut self) -> &mut T {
        self.components
            .get_mut(&TypeId::of::<T>())
            .unwrap()
            .downcast_mut::<T>()
            .unwrap()
    }

    pub fn try_component<T: Component>(&self) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|c| c.as_ref())?
            .downcast_ref::<T>()
    }
    pub fn try_component_mut<T: Component>(&mut self) -> Option<&mut T> {
        self.components
            .get_mut(&TypeId::of::<T>())?
            .downcast_mut::<T>()
    }

    pub fn add_component<T: Component>(&mut self, component: Box<T>) {
        let type_id = TypeId::of::<T>();
        self.components.insert(TypeId::of::<T>(), component);
        self.components.get_mut(&type_id).unwrap().on_added(self.id)
    }
}

pub trait Component: Downcast + Debug + Sync + Send {
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

    pub fn register_object(&mut self, object: Object) -> u64 {
        let idx = self.obj_index;
        self.objects.insert(idx, object);

        let object = self.get_object_mut(idx);
        object.receive_id(idx);
        for cmp in self.get_object_mut(idx).components.values_mut() {
            cmp.on_added(idx)
        }

        self.obj_index += 1;
        idx
    }

    pub fn get_object(&self, index: u64) -> &Object {
        self.objects.get(&index).unwrap()
    }

    pub fn get_object_mut(&mut self, index: u64) -> &mut Object {
        self.objects.get_mut(&index).unwrap()
    }

    pub fn try_get_object(&self, index: u64) -> Option<&Object> {
        self.objects.get(&index)
    }

    pub fn try_get_object_mut(&mut self, index: u64) -> Option<&mut Object> {
        self.objects.get_mut(&index)
    }

    pub fn objects_iter(&self) -> impl Iterator<Item = &Object> {
        self.objects.values()
    }

    pub fn objects_iter_mut(&mut self) -> impl Iterator<Item = &mut Object> {
        self.objects.values_mut()
    }
}
