use nohash_hasher::*;
use bimap::BiMap;
use std::ops::Index;
use std::collections::hash_map::Keys;
use compact_str::CompactString;


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


impl<T> BaseContainer<T> {
    pub fn new(elements_list_ : IntMap<usize, T>, element_names_ : BiMap<usize, CompactString>) -> BaseContainer<T>{
        BaseContainer { elements_list_, element_names_}
    }
    pub fn elements(&self) -> &IntMap<usize, T>{
        &self.elements_list_
    }
    pub fn names(&self) -> &BiMap<usize, CompactString>{
        &self.element_names_
    }
    pub fn get(&self, id : &usize) -> Option<&T> {
        self.elements_list_.get(id)
    }
    pub fn keys(&self) -> Keys<'_, usize, T> {
        self.elements_list_.keys()
    }
    pub fn get_mut(&mut self, id : &usize) -> Option<&mut T> {
        self.elements_list_.get_mut(id)
    }
    pub fn insert(&mut self, id : usize, element : T, name : &str) -> Option<T>{
        let output = self.elements_list_.insert(id, element);
        let _ = self.element_names_.insert(id, name.into());
        output
    }
    pub fn insert_str(&mut self, id : usize, element : T, name : CompactString) -> Option<T>{
        let output = self.elements_list_.insert(id, element);
        let _ = self.element_names_.insert(id, name);
        output
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
    pub fn remove(&mut self, id : &usize) -> Option<CompactString>{
        self.elements_list_.remove(id);
        match self.element_names_.remove_by_left(id){
            Some((_, name)) => Some(name),
            None => None
        }
    }
    pub fn remove_by_name(&mut self, name : &str) -> Option<usize>{
        
        let id =self.element_names_.remove_by_right(name)?;
        self.elements_list_.remove(&id.0);
        Some(id.0)
    }
    pub fn pop(&mut self, id : usize) -> T{
        self.element_names_.remove_by_left(&id);
        self.elements_list_.remove_entry(&id).unwrap().1
    }
    pub fn extend(&mut self, mut other : BaseContainer<T>) -> Vec<Option<T>>{
        let mut output = Vec::with_capacity(other.len());
        for (id, name) in other.element_names_.clone(){
            output.push(self.insert(id, other.pop(id), &name));
        }
        output
    }
    pub fn extend_no_overwrite(&mut self, mut other : BaseContainer<T>){
        for (id, name) in other.element_names_.clone(){
            self.insert_no_overwrite(id, other.pop(id), &name);
        }
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

impl<T> std::iter::Iterator for &BaseContainer<T> where T : Clone{
    type Item = (usize, CompactString, T);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let output = self.elements().iter().next();
        match output {
            Some(x) => Some((*x.0, self.get_name(x.0).unwrap().into(),x.1.clone())),
            None => None
        }
    }
}