use nohash_hasher::*;
use bimap::{BiMap, Overwritten::Left};
use std::sync::{Weak, Mutex, Arc};
use std::ops::Index;
use std::collections::hash_map::Keys;
use compact_str::CompactString;

use crate::oe::types::object_trait::*;
use crate::oe::types::polygonstorage::StaticPolygonStorage;
use crate::oe::types::polygonstoragetrait::PolygonStorageTrait;

#[derive(Debug, Clone)]
pub struct BaseContainer<T> {
    elements_list_ : IntMap<usize, T>,
    element_names_ : BiMap<usize, CompactString>,
}

impl<T> Default for BaseContainer<T> {
    fn default() -> Self {
        BaseContainer{
            element_names_ : Default::default(),
            elements_list_ : Default::default()
        }
    }
}

impl<T> BaseContainer<Weak<Mutex<T>>>{
    pub fn cleanup(&mut self){
        let elements : Vec<usize> = self.elements_list_.iter().map(|(x, _)| *x).collect();
        for id in elements{
            let elem = self.elements_list_[&id].upgrade();
            match elem {
                Some(x) => continue,
                None => self.remove(&id),
            }
        }
    }
    pub fn get_strong_elements(&self) -> IntMap<usize, Arc<Mutex<T>>> {
        self.elements_list_.iter().map(|(id, element)| {(*id, element.upgrade().unwrap())}).collect()
    }
}

impl<T> BaseContainer<Weak<Mutex<T>>> where T : Clone{
    pub fn get_real_elements(&self) -> IntMap<usize, T> {
        self.elements_list_.iter().map(|(id, element)| {
            let arced = element.upgrade().unwrap();
            let locked = arced.lock().unwrap();
            (*id, locked.clone())
        }).collect()
    }

    pub fn get_real(&self) -> BaseContainer<T> {
        BaseContainer { elements_list_: self.get_real_elements(), element_names_: self.element_names_.clone()}
    }
}

impl BaseContainer<Weak<Mutex<Box<dyn ObjectTrait>>>>{
    pub fn get_real_elements(&self) -> IntMap<usize, Box<dyn ObjectTrait>> {
        self.elements_list_.iter()
         .map(|(id, element)| {
            let arced = element.upgrade().unwrap();
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
        BaseContainer { elements_list_: self.get_real_elements(), element_names_: self.element_names_.clone()}
    }
}

impl BaseContainer<Weak<Mutex<Box<dyn PolygonStorageTrait>>>>{
    pub fn get_real_elements(&self) -> IntMap<usize, StaticPolygonStorage> {
        self.elements_list_.iter()
         .map(|(id, element)| {
            let arced = element.upgrade().unwrap();
            let locked = arced.lock().unwrap();
            (*id, StaticPolygonStorage{data : locked.get_data().unwrap().clone()})
        }).collect()
    }

    pub fn get_real(&self) -> BaseContainer<StaticPolygonStorage> {
        BaseContainer { elements_list_: self.get_real_elements(), element_names_: self.element_names_.clone()}
    }
}


impl<T> BaseContainer<T> {

    pub fn get(&self, id : &usize) -> Option<&T> {
        self.elements_list_.get(id)
    }
    pub fn keys(&self) -> Keys<'_, usize, T> {
        self.elements_list_.keys()
    }
    pub fn get_mut(&mut self, id : &usize) -> Option<&mut T> {
        self.elements_list_.get_mut(id)
    }
    pub fn insert(&mut self, id : usize, element : T, name : &str) -> usize{
        self.elements_list_.insert(id, element);
        let old_id = self.element_names_.insert(id, name.into());
        match old_id {
            Left(l, _) => l,
            _ => 0
        }
    }
    pub fn insert_no_overwrite(&mut self, id : usize, element : T, name : &str) -> bool{
        if !self.contains_name(name){
            self.insert(id, element, name);
            true
        }
        else{
            false
        }
    }
    pub fn clear(&mut self){
        self.elements_list_.clear();
        self.element_names_.clear();
    }
    pub fn len(&self) -> usize{
        self.elements_list_.len()
    }
    pub fn contains(&self, id : &usize) -> bool{
        self.elements_list_.contains_key(id)
    }
    pub fn contains_name(&self, name : &str) -> bool{
        self.element_names_.contains_right(name)
    }
    pub fn get_id(&self, name : &str) -> Option<usize> {
        Some(*self.element_names_.get_by_right(name)?)
    }
    pub fn get_name(&self, id : &usize) -> Option<&str> {
        Some(self.element_names_.get_by_left(id)?)
    }
    pub fn remove(&mut self, id : &usize){
        self.elements_list_.remove(id);
        self.element_names_.remove_by_left(id);
    }
}

impl<T> Index<usize> for BaseContainer<T> {
    type Output = T;
    fn index(&self, id : usize) -> &Self::Output {
        &self.elements_list_[&id]
    }
}

impl<T> Index<&str> for BaseContainer<T> {
    type Output = T;
    fn index(&self, name : &str) -> &Self::Output {
        let id = self.get_id(name).unwrap();
        &self.elements_list_[&id]
    }
}