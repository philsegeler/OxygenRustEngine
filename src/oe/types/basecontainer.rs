use nohash_hasher::*;
use bimap::{BiMap, Overwritten::Left};
use std::ops::Index;
use std::collections::hash_map::Keys;

#[derive(Debug, Clone)]
pub struct BaseContainer<T> {
    elements_list_ : IntMap<usize, T>,
    element_names_ : BiMap<usize, String>,
}

impl<T> Default for BaseContainer<T> {
    fn default() -> Self {
        BaseContainer{
            element_names_ : Default::default(),
            elements_list_ : Default::default()
        }
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
        let old_id = self.element_names_.insert(id, name.to_owned());
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
    pub fn get_id(&self, name : &str) -> Option<&usize> {
        Some(self.element_names_.get_by_right(name)?)
    }
    pub fn get_name(&self, id : &usize) -> Option<&str> {
        Some(self.element_names_.get_by_left(id)?)
    }
    pub fn remove(&mut self, id : &usize){
        self.elements_list_.remove(id);
        self.element_names_.remove_by_left(id);
    }
}

impl<T> Index<&usize> for BaseContainer<T> {
    type Output = T;
    fn index(&self, id : &usize) -> &Self::Output {
        &self.elements_list_[id]
    }
}

impl<T> Index<&str> for BaseContainer<T> {
    type Output = T;
    fn index(&self, name : &str) -> &Self::Output {
        let id = self.get_id(name).unwrap();
        &self.elements_list_[id]
    }
}