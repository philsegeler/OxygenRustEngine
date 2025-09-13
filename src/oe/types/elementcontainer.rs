use std::sync::{Arc, Mutex, MutexGuard};
use std::iter::Iterator;
use std::collections::HashSet;
use std::ops::Index;
//use std::iter::Iterator;

use super::object_trait::*;
use super::polygonstorage::RendererPolygonStorage;
use super::polygonstoragetrait::*;
use super::basecontainer::*;
use compact_str::CompactString;
//use no_deadlocks::RwLockReadGuard;
use nohash_hasher::*;

pub type SingleElement<T> = Mutex<(T, bool)>; 


// ELEMENT SNAPSHOT
#[derive(Debug, Default)]
pub struct ElementSnapshot<T>{
    data : BaseContainer<T>,
    deleted : HashSet<CompactString>
}

impl<T> ElementSnapshot<T> {
    pub fn get_data(&self) -> &BaseContainer<T>{
        &self.data
    }
    pub fn get_deleted(&self) -> &HashSet<CompactString>{
        &self.deleted
    }
    pub fn take_data(&mut self) -> BaseContainer<T>{
        std::mem::take(&mut self.data)
    }
    pub fn take_deleted(&mut self) -> HashSet<CompactString>{
        std::mem::take(&mut self.deleted)
    }
}

impl<T> std::ops::Index<&str> for ElementSnapshot<T>{
    type Output = T;
    fn index(&self, index: &str) -> &Self::Output {
        &self.data[index]
    }
}

// ELEMENT CONTAINER
#[derive(Clone, Debug)]
pub struct ElementContainer<T>{
    data : BaseContainer<Arc<SingleElement<T>>>,
    deleted : HashSet<CompactString>,
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
    pub fn contains(&self, id : &usize) -> bool{
        self.data.contains(id)
    }
    pub fn contains_name(&self, name : &str) -> bool{
        self.data.contains_name(name)
    }
    pub fn contains_names<'a>(&self, names : impl Iterator<Item=&'a CompactString>) -> bool{
        self.data.contains_names(names)
    }

    pub fn get_name(&self, id : &usize) -> Option<&str> {
        Some(self.data.get_name(id)?)
    }
    pub fn names(&self) -> Vec<CompactString>{
        self.data.names().iter().map(|(_, x)| x.clone()).collect()
    }

    pub fn get_id(&self, name : &str) -> Option<usize> {
        Some(self.data.get_id(name)?)
    }
    pub fn insert(&mut self, id : usize, element : Arc<SingleElement<T>>, name : &str) -> Option<Arc<SingleElement<T>>>{
        if self.deleted.contains(name){
            self.deleted.remove(name);
        }
        self.data.insert(id, element, name).0
        
    }
    pub fn insert_str(&mut self, id : usize, element : Arc<SingleElement<T>>, name : CompactString) -> Option<Arc<SingleElement<T>>>{
        self.data.insert_str(id, element, name).0
    }

    pub fn remove(&mut self, id : usize){
        self.deleted.insert(self.get_name(&id).unwrap().into());
    }

    pub fn remove_now(&mut self, id : &usize) -> Option<(Arc<SingleElement<T>>, CompactString)>{
        self.deleted.insert(self.get_name(&id).unwrap().into());
        self.data.remove(id)
    }

    pub fn update(&mut self){

        for name in self.deleted.drain(){
            self.data.remove_by_name(&name);
        }

    }

    pub fn get_strong_elements(&self) -> IntMap<usize, Arc<SingleElement<T>>> {
        self.data.elements().iter().map(|(id, element)| {(*id, element.clone())}).collect()
    }
}

impl<'a, T> std::iter::IntoIterator for &'a ElementContainer<T>{
    type IntoIter = BaseContainerIntoIter<'a, &'a Arc<SingleElement<T>>>;
    type Item = (&'a usize, &'a str, &'a Arc<SingleElement<T>>);
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
    fn get_deleted(&mut self) -> &mut HashSet<CompactString>;

}
impl<T> GetDataElementContainer for ElementContainer<T> {
    type InternalType=T;
    fn get_data(&self) -> &BaseContainer<Arc<SingleElement<Self::InternalType>>>{
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut BaseContainer<Arc<SingleElement<Self::InternalType>>>{
        &mut self.data
    }
    fn get_deleted(&mut self) -> &mut HashSet<CompactString>{
        &mut self.deleted
    }
}

pub trait ChangedElements : GetDataElementContainer {
    // functions to implement
    type ProcessOutputType;
    fn process(&self, locked : MutexGuard<'_, (Self::InternalType, bool)>) -> Self::ProcessOutputType;

    // derived functions
    fn get_changed_elements(&self, changed : bool) -> IntMap<usize, Self::ProcessOutputType> {
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

    fn get_changed_elements_and_reset(&self, changed : bool) -> IntMap<usize, Self::ProcessOutputType> {
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

    fn get_changed(&mut self, changed : bool) -> ElementSnapshot<Self::ProcessOutputType> {

        let deleted = self.get_deleted().clone();
        let data = BaseContainer::new( self.get_changed_elements(changed), self.get_data().names().clone());
        ElementSnapshot { data, deleted }
    }
    fn get_changed_and_reset(&mut self, changed : bool) -> ElementSnapshot<Self::ProcessOutputType> {
        let deleted = self.get_deleted().clone();
        /*for name in &deleted{
            self.get_data_mut().remove_by_name(name);
        }*/
        let data = BaseContainer::new( self.get_changed_elements_and_reset(changed), self.get_data().names().clone());
        ElementSnapshot { data, deleted }
    }
}

impl<T> ChangedElements for ElementContainer<T> where T : Clone{
    type ProcessOutputType = Self::InternalType;
    fn process(&self, locked : MutexGuard<'_, (Self::InternalType, bool)>) -> Self::ProcessOutputType{
        locked.0.clone()
    }
}



impl ChangedElements for ElementContainer<Box<dyn ObjectTrait>> {
    type ProcessOutputType = ChangedObjectEnum;
    fn process(&self, locked : MutexGuard<'_, (Self::InternalType, bool)>) -> Self::ProcessOutputType{
        let output = match locked.0.get_type(){
                ObjectType::Camera => Some(ChangedObjectEnum::Camera(locked.0.get_camera().unwrap())),
                ObjectType::Light => Some(ChangedObjectEnum::Light(locked.0.get_light().unwrap())),
                ObjectType::Mesh => Some(ChangedObjectEnum::Mesh(locked.0.get_mesh().unwrap())),
                _ => None
        };
        output.unwrap()
    }
}

impl ChangedElements for ElementContainer<Box<dyn PolygonStorageTrait>> {
    type ProcessOutputType = RendererPolygonStorage;
    fn process(&self, locked : MutexGuard<'_, (Self::InternalType, bool)>) -> Self::ProcessOutputType{
        RendererPolygonStorage{data : Some(locked.0.get_data().unwrap().clone()), ps_type : locked.0.get_type()}
    }
}