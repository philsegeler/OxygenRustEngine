use std::sync::{Arc, Mutex, Weak};

use super::object_trait::*;
use super::polygonstorage::RendererPolygonStorage;
use super::polygonstoragetrait::*;
use super::basecontainer::*;
//use no_deadlocks::RwLockReadGuard;
use nohash_hasher::*;


// ELEMENT CONTAINER
#[derive(Default, Clone)]
pub struct ElementContainer<T>{
    data : BaseContainer<(T, bool)>
}

impl<T> ElementContainer<T>{
    pub fn ids(&self) -> Vec<usize> {
        self.data.keys().cloned().collect()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn contains(&self, event_id : &usize) -> bool{
        self.data.contains(event_id)
    }
    pub fn contains_name(&self, event_name : &str) -> bool{
        self.data.contains_name(event_name)
    }

    pub fn get_name(&self, event_id : &usize) -> Option<&str> {
        Some(self.data.get_name(event_id)?)
    }

    pub fn get_id(&self, event_name : &str) -> Option<usize> {
        Some(self.data.get_id(event_name)?)
    }
    pub fn insert(&mut self, id : usize, element : T, name : &str) -> usize{
        self.data.insert(id, (element, true), name)
    }
}

impl<T> ElementContainer<Weak<Mutex<T>>>{
    pub fn cleanup(&mut self){
        let elements : Vec<usize> = self.data.elements().iter().map(|(x, _)| *x).collect();
        for id in elements{
            let elem = self.data[id].0.upgrade();
            match elem {
                Some(_) => continue,
                None => {
                    self.data.remove(&id);
                },
            }
        }
    }
    pub fn get_strong_elements(&self) -> IntMap<usize, Arc<Mutex<T>>> {
        self.data.elements().iter().map(|(id, element)| {(*id, element.0.upgrade().unwrap())}).collect()
    }
}

impl<T> ElementContainer<Weak<Mutex<T>>> where T : Clone{
    pub fn get_real_elements(&self) -> IntMap<usize, T> {
        self.data.elements().iter().map(|(id, element)| {
            let arced = element.0.upgrade().unwrap();
            let locked = arced.lock().unwrap();
            (*id, locked.clone())
        }).collect()
    }

    pub fn get_real(&self) -> BaseContainer<T> {
        BaseContainer::new( self.get_real_elements(), self.data.names().clone())
    }
}

impl ElementContainer<Weak<Mutex<Box<dyn ObjectTrait>>>>{
    pub fn get_real_elements(&self) -> IntMap<usize, Box<dyn ObjectTrait>> {
        self.data.elements().iter()
         .map(|(id, element)| {
            let arced = element.0.upgrade().unwrap();
            let locked = arced.lock().unwrap();
            let output = match locked.get_type(){
                ObjectType::Camera => Some(Box::new(locked.get_camera().unwrap()) as Box<dyn ObjectTrait>),
                ObjectType::Light => Some(Box::new(locked.get_light().unwrap()) as Box<dyn ObjectTrait>),
                ObjectType::Mesh => Some(Box::new(locked.get_mesh().unwrap()) as Box<dyn ObjectTrait>),
                _ => None
            };
            (*id, output.unwrap())
        }).collect()
    }

    pub fn get_real(&self) -> BaseContainer<Box<dyn ObjectTrait>> {
        BaseContainer::new( self.get_real_elements(), self.data.names().clone())
    }
}

impl ElementContainer<Weak<Mutex<Box<dyn PolygonStorageTrait>>>>{
    pub fn get_real_elements(&self, changed : bool) -> IntMap<usize, RendererPolygonStorage> {
        self.data.elements().iter()
         .map(|(id, element)| {
            let arced = element.0.upgrade().unwrap();
            let locked = arced.lock().unwrap();
            if changed || locked.has_changed() {
                (*id, RendererPolygonStorage{data : Some(locked.get_data().unwrap().clone())})
            }
            else {
                (*id, RendererPolygonStorage{data : None})
            }
        }).collect()
    }

    pub fn get_real(&self, changed : bool) -> BaseContainer<RendererPolygonStorage> {
        BaseContainer::new( self.get_real_elements(changed), self.data.names().clone())
    }
}