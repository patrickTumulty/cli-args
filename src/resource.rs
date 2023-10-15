use std::collections::HashMap;
use std::any::{Any, TypeId};

pub struct FlagResource {
    pub flag_value_map: HashMap<String, String>
}

impl FlagResource {
    pub fn new() -> Self { 
        Self { 
            flag_value_map: HashMap::new() 
        } 
    }

}

pub struct ResContainer {
    resource_map: HashMap<TypeId, Box<dyn Any>>,
}

impl ResContainer {
    pub fn new() -> ResContainer {
        ResContainer {
            resource_map: HashMap::new()
        }
    }

    pub fn add<V: 'static>(&mut self, resource: V) {
        self.resource_map.insert(resource.type_id(), Box::new(resource));
    }

    pub fn get<V: 'static>(&mut self) -> Option<&mut V> {
        return if let Some(resource) = self.resource_map.get_mut(&TypeId::of::<V>()) {
            resource.as_mut().downcast_mut::<V>()
        } else {
            None
        }
    }
}
