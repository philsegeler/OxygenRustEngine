use std::sync::{Arc, Mutex, MutexGuard};
use std::iter::Iterator;
use std::ops::Index;

use super::object_trait::*;
use super::polygonstorage::RendererPolygonStorage;
use super::polygonstoragetrait::*;
use super::basecontainer::*;
use compact_str::CompactString;
//use no_deadlocks::RwLockReadGuard;
use nohash_hasher::*;

pub type SingleElement<T> = Mutex<(T, bool)>; 


// ELEMENT SNAPSHOT
#[derive(Debug)]
pub struct ElementSnapshot<T>{
    data : BaseContainer<T>,
    deleted : Vec<CompactString>
}

impl<T> Default for ElementSnapshot<T> {
    fn default() -> Self {
        ElementSnapshot{
            data : Default::default(),
            deleted : Default::default()
        }
    }
}

// ELEMENT CONTAINER
#[derive(Clone, Debug)]
pub struct ElementContainer<T>{
    data : BaseContainer<Arc<SingleElement<T>>>,
    deleted : Vec<CompactString>,
}

impl<T> Default for ElementContainer<T> {
    fn default() -> Self {
        ElementContainer{
            data : Default::default(),
            deleted : Default::default(),
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
    pub fn names(&self) -> Vec<CompactString>{
        self.data.names().iter().map(|(_, x)| x.clone()).collect()
    }

    pub fn get_id(&self, event_name : &str) -> Option<usize> {
        Some(self.data.get_id(event_name)?)
    }
    pub fn insert(&mut self, id : usize, element : Arc<SingleElement<T>>, name : &str) -> Option<Arc<SingleElement<T>>>{
        self.data.insert(id, element, name)
    }
    pub fn insert_str(&mut self, id : usize, element : Arc<SingleElement<T>>, name : CompactString) -> Option<Arc<SingleElement<T>>>{
        self.data.insert_str(id, element, name)
    }

    pub fn remove(&mut self, id : usize){
        self.deleted.push(self.get_name(&id).unwrap().into());
    }

    pub fn remove_now(&mut self, id : &usize) -> Option<(Arc<SingleElement<T>>, CompactString)>{
        self.data.remove(id)
    }

    pub fn update(&mut self){

        for name in self.deleted.drain(..){
            self.data.remove_by_name(&name);
        }

    }

    pub fn get_strong_elements(&self) -> IntMap<usize, Arc<SingleElement<T>>> {
        self.data.elements().iter().map(|(id, element)| {(*id, element.clone())}).collect()
    }
}

impl<T> std::iter::IntoIterator for &ElementContainer<T>{
    type IntoIter = BaseContainerIntoIter<Arc<SingleElement<T>>>;
    type Item = (usize, CompactString, Arc<SingleElement<T>>);
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> Index<usize> for ElementContainer<T> {
    type Output = Arc<SingleElement<T>>;
    fn index(&self, id : usize) -> &Self::Output {
        &self.data[id]
    }
}

impl<T> Index<&str> for ElementContainer<T> {
    type Output = Arc<SingleElement<T>>;
    fn index(&self, name : &str) -> &Self::Output {
        &self.data[name]
    }
}

pub trait GetDataElementContainer{
    type InternalType;
    fn get_data(&self) -> &BaseContainer<Arc<SingleElement<Self::InternalType>>>;
    fn get_data_mut(&mut self) -> &mut BaseContainer<Arc<SingleElement<Self::InternalType>>>;
    fn get_deleted(&mut self) -> &mut Vec<CompactString>;

}
impl<T> GetDataElementContainer for ElementContainer<T> {
    type InternalType=T;
    fn get_data(&self) -> &BaseContainer<Arc<SingleElement<Self::InternalType>>>{
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut BaseContainer<Arc<SingleElement<Self::InternalType>>>{
        &mut self.data
    }
    fn get_deleted(&mut self) -> &mut Vec<CompactString>{
        &mut self.deleted
    }
}

pub trait ChangedElements : GetDataElementContainer {
    // functions to implement
    fn process(&self, locked : MutexGuard<'_, (Self::InternalType, bool)>) -> Self::InternalType;

    // derived functions
    fn get_changed_elements(&self, changed : bool) -> IntMap<usize, Self::InternalType> {
        self.get_data().elements().iter().filter_map(|(id, element)| {
            let arced = element;
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
            let arced = element;
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

    fn get_changed(&mut self, changed : bool) -> ElementSnapshot<Self::InternalType> {

        let deleted = self.get_deleted().clone();
        let data = BaseContainer::new( self.get_changed_elements(changed), self.get_data().names().clone());
        ElementSnapshot { data, deleted }
    }
    fn get_changed_and_reset(&mut self, changed : bool) -> ElementSnapshot<Self::InternalType> {
        let deleted = self.get_deleted().clone();
        for name in &deleted{
            self.get_data_mut().remove_by_name(name);
        }
        let data = BaseContainer::new( self.get_changed_elements_and_reset(changed), self.get_data().names().clone());
        ElementSnapshot { data, deleted }
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
        Box::new(RendererPolygonStorage{data : Some(locked.0.get_data().unwrap().clone()), ps_type : locked.0.get_type()}) as Box<dyn PolygonStorageTrait>
    }
}