use std::ops::Index;

use compact_str::CompactString;
use nohash_hasher::IntMap;

use super::super::super::types::basecontainer::BaseContainer;

#[derive(Debug, Default)]
pub struct RenderDataContainer<T>{
    data : BaseContainer<T>,
    deleted : Vec<T>
}

impl<T> RenderDataContainer<T>{
    pub fn new() -> RenderDataContainer<T>{
        RenderDataContainer { data: Default::default(), deleted: Default::default() }
    }
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
    pub fn get_mut(&mut self, id : &usize) -> Option<&mut T>{
        self.data.get_mut(id)
    }
    pub fn insert(&mut self, id : usize, element : T, name : &str) -> Option<usize>{
        self.insert_str(id, element, name.into())
    }
    pub fn insert_str(&mut self, id : usize, element : T, name : CompactString) -> Option<usize>{
        let (elem, old_id) = self.data.insert_str(id, element, name);
        
        // delete only if ids did not match
        if let Some(prev_id) = old_id{
            if prev_id != id{
                self.deleted.push(elem.unwrap());
                return old_id;
            }
        }
        None
    }

    pub fn remove(&mut self, id : usize){
        self.deleted.push(self.data.remove(&id).unwrap().0);
    }
    pub fn remove_by_name(&mut self, name : &str){
        if self.contains_name(name){
            self.remove(self.get_id(name).unwrap());
        }
    }

    pub fn update(&mut self, delete_all : bool){
        self.deleted.clear();
        if delete_all{
            self.data.clear();
        }
    }
}

impl<T> Index<usize> for RenderDataContainer<T> {
    type Output = T;
    fn index(&self, id : usize) -> &Self::Output {
        &self.data[id]
    }
}

impl<'a, T> std::iter::IntoIterator for &'a RenderDataContainer<T>{
    type IntoIter = <&'a IntMap<usize, T> as IntoIterator>::IntoIter;
    type Item = (&'a usize, &'a T);
    fn into_iter(self) -> Self::IntoIter {
        self.data.elements().into_iter()
    }
}