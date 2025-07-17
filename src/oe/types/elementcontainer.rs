use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::object_trait::*;
use super::polygonstorage::RendererPolygonStorage;
use super::polygonstoragetrait::*;
use super::basecontainer::*;
//use no_deadlocks::RwLockReadGuard;
use nohash_hasher::*;


// ELEMENT CONTAINER
#[derive(Clone, Debug)]
pub struct ElementContainer<T>{
    data : BaseContainer<Weak<Mutex<(T, bool)>>>
}

impl<T> Default for ElementContainer<T> {
    fn default() -> Self {
        ElementContainer{
            data : Default::default(),
        }
    }
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
    pub fn insert(&mut self, id : usize, element : Weak<Mutex<(T, bool)>>, name : &str) -> usize{
        self.data.insert(id, element, name)
    }
    pub fn cleanup(&mut self){
        let elements : Vec<usize> = self.data.elements().iter().map(|(x, _)| *x).collect();
        for id in elements{
            let elem = self.data[id].upgrade();
            match elem {
                Some(_) => continue,
                None => {
                    self.data.remove(&id);
                },
            }
        }
    }
    pub fn get_strong_elements(&self) -> IntMap<usize, Arc<Mutex<(T, bool)>>> {
        self.data.elements().iter().map(|(id, element)| {(*id, element.upgrade().unwrap())}).collect()
    }
}

pub trait GetDataElementContainer{
    type InternalType;
    fn get_data(&self) -> &BaseContainer<Weak<Mutex<(Self::InternalType, bool)>>>;
}

impl<T> GetDataElementContainer for ElementContainer<T> {
    type InternalType=T;
    fn get_data(&self) -> &BaseContainer<Weak<Mutex<(Self::InternalType, bool)>>>{
        &self.data
    }
}

pub trait ChangedElements : GetDataElementContainer {
    // functions to implement
    fn process(&self, locked : MutexGuard<'_, (Self::InternalType, bool)>) -> Self::InternalType;

    // derived functions
    fn get_changed_elements(&self, changed : bool) -> IntMap<usize, Self::InternalType> {
        self.get_data().elements().iter().filter_map(|(id, element)| {
            let arced = element.upgrade().unwrap();
            let locked = arced.lock().unwrap();
            if changed || locked.1 {
                Some((*id, self.process(locked)))
            }
            else {
                None
            }
        }).collect()
    }

    fn get_changed_elements_and_reset(&self, changed : bool) -> IntMap<usize, Self::InternalType> {
        self.get_data().elements().iter().filter_map(|(id, element)| {
            let arced = element.upgrade().unwrap();
            let mut locked = arced.lock().unwrap();
            if changed || locked.1 {
                locked.1 = false;
                Some((*id, self.process(locked)))
            }
            else {
                None
            }
        }).collect()
    }

    fn get_changed(&self, changed : bool) -> BaseContainer<Self::InternalType> {
        BaseContainer::new( self.get_changed_elements(changed), self.get_data().names().clone())
    }
}

impl<T> ChangedElements for ElementContainer<T> where T : Clone{
    fn process(&self, locked : MutexGuard<'_, (Self::InternalType, bool)>) -> Self::InternalType{
        locked.0.clone()
    }
}

impl ChangedElements for ElementContainer<Box<dyn ObjectTrait>> {
    fn process(&self, locked : MutexGuard<'_, (Self::InternalType, bool)>) -> Self::InternalType{
        let output = match locked.0.get_type(){
                ObjectType::Camera => Some(Box::new(locked.0.get_camera().unwrap()) as Box<dyn ObjectTrait>),
                ObjectType::Light => Some(Box::new(locked.0.get_light().unwrap()) as Box<dyn ObjectTrait>),
                ObjectType::Mesh => Some(Box::new(locked.0.get_mesh().unwrap()) as Box<dyn ObjectTrait>),
                _ => None
        };
        output.unwrap()
    }
}

impl ChangedElements for ElementContainer<Box<dyn PolygonStorageTrait>> {
    fn process(&self, locked : MutexGuard<'_, (Self::InternalType, bool)>) -> Self::InternalType{
        Box::new(RendererPolygonStorage{data : Some(locked.0.get_data().unwrap().clone())}) as Box<dyn PolygonStorageTrait>
    }
}