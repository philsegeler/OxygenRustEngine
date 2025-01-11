use super::types::basecontainer::*;
use nohash_hasher::*;
use std::sync::Mutex;
use std::sync::Arc;
//use no_deadlocks::Mutex;
//use parking_lot::Mutex;
use std::ops::Index;
use std::time;
use std::sync::atomic::{AtomicU32, Ordering};
use rayon::prelude::*;

use super::event::*;

// EVENT CONTAINER

#[derive(Default)]
pub struct EventContainer<'a>{
    events_list_ : BaseContainer<Event<'a>>,
    pending_events_ : Mutex<Vec<usize>>,
    repeated_events_ : Mutex<IntMap<usize, Option<f32>>>,
    pub happened_events_counter_ : Arc<IntMap<usize, AtomicU32>>,
}

impl<'a> EventContainer<'a> {

    pub fn ids(&self) -> Vec<usize> {
        self.events_list_.keys().cloned().collect()
    }
    pub fn len(&self) -> usize {
        self.events_list_.len()
    }
    pub fn contains(&self, event_id : &usize) -> bool{
        self.events_list_.contains(event_id)
    }
    pub fn contains_name(&self, event_name : &str) -> bool{
        self.events_list_.contains_name(event_name)
    }

    pub fn get_name(&self, event_id : &usize) -> Option<&str> {
        Some(self.events_list_.get_name(event_id)?)
    }

    pub fn get_id(&self, event_name : &str) -> Option<usize> {
        Some(self.events_list_.get_id(event_name)?)
    }

    pub fn insert_no_overwrite(&mut self, name : &str, func : Box<dyn EventFuncTrait + 'a>, event_type : EventEnum) -> usize{
        if !self.events_list_.contains_name(name) {
            let new_event = Event::new(Some(func), event_type);
            let output = new_event.id();
            let events_list = &mut self.events_list_;
            events_list.insert(new_event.id(), new_event, name);
            return output;
        }
        else{
            return 0;
        }
    }

    pub fn insert(&mut self, name : &str, func : Box<dyn EventFuncTrait + 'a>, event_type : EventEnum) -> usize {
        let events_list = &mut self.events_list_;
        if events_list.contains_name(name) {
            let event_id = events_list.get_id(name).unwrap();
            let new_event  = events_list[&event_id].clone();
            events_list.insert(event_id, new_event, name);
            return event_id;
        }
        else{
            let new_event = Event::new(Some(func), event_type);
            let output = new_event.id();
            events_list.insert(new_event.id(), new_event, name);
            return output;
        }
    }

    pub fn set_func(&self, event_id : &usize, func : Box<dyn EventFuncTrait + 'a>, data : Box<dyn EventDataTrait>) -> Option<bool>{
        let events_list = &self.events_list_;
        events_list.get(event_id)?.set_func(func, data);
        Some(true)
    }

    pub fn set_func_data(&self, event_id : &usize, data : Box<dyn EventDataTrait>) -> Option<bool>{
        let events_list = &self.events_list_;
        events_list.get(event_id)?.set_func_data(data);
        Some(true)
    }

    pub fn update_event(&mut self, event_id : &usize, updated_time : &time::Instant) -> Option<bool>{
        let events_list = &mut self.events_list_;
        events_list.get_mut(event_id)?.update(updated_time);
        Some(true)            
    }

    pub fn remove(&mut self, event_id : usize){
        let events_list = &mut self.events_list_;
        events_list.remove(&event_id);
    }

    pub fn broadcast(&self, event_id : &usize) -> Option<bool>{
        if self.events_list_.contains(event_id){
            self.pending_events_.lock().unwrap().push(*event_id);
            return Some(true);
        }
        else{
            return None;
        }
    }

    pub fn repeat(&self, event_id : &usize, interval : Option<f32>) -> Option<bool>{
        if self.events_list_.contains(event_id){
            let mut event_set = self.repeated_events_.lock().unwrap();
            event_set.insert(*event_id, interval);
            return Some(true);
        }
        else{
            return None;
        }
    }
    pub fn derepeat(&self, event_id : &usize) -> Option<bool>{
        if self.events_list_.contains(event_id){
            let mut event_set = self.repeated_events_.lock().unwrap();
            event_set.remove(event_id);
            return Some(true);
        }
        else{
            return None;
        }
    }

    pub fn get_repeated(&self) -> IntMap<usize, Option<f32>>{
        let output = self.repeated_events_.lock().unwrap();
        output.clone()
    }

    pub fn pop_pending(&self) -> Vec<usize> {
        let mut pending_events = self.pending_events_.lock().unwrap();
        let output = pending_events.clone();
        pending_events.clear();
        output
    }

    pub fn consume(&self) -> EventContainer<'a> {
        let mut pending_events = self.pending_events_.lock().unwrap();
        let pending_events_copy = pending_events.clone();
        pending_events.clear();
        let repeated_events = self.repeated_events_.lock().unwrap();
        EventContainer{
            events_list_ : self.events_list_.clone(),
            repeated_events_ : Mutex::new(repeated_events.clone()),
            pending_events_ : Mutex::new(pending_events_copy),
            happened_events_counter_ : Arc::clone(&self.happened_events_counter_)
        }
    }
        // HAS TO BE READ ONLY
    pub fn handle_all_events(&self, updated_time : &time::Instant) -> Option<bool>{

        //let pool = ThreadPoolBuilder::new().build().unwrap();

        for key in self.happened_events_counter_.keys(){
            self.happened_events_counter_[key].store(0, Ordering::Relaxed);
        }

        //for event_id in self.events_.pop_pending() {
            
        let pending_events = self.pop_pending();
        
        pending_events.par_iter().for_each(|event_id| {
            self.execute_events(&[*event_id], updated_time, None).unwrap();

        });

        //for (event_id, interval) in self.events_.get_repeated() {
        let repeated_events = self.repeated_events_.lock().unwrap();
        repeated_events.par_iter().for_each(|(event_id, interval)|{
            //println!("Before deadlock {:?}", self.get_event_name(event_id));
            self.execute_events(&[*event_id], updated_time, *interval).unwrap();
        });
        
        Some(true)
    }
    pub fn execute_events(&self, event_list: &[usize], updated_time : &time::Instant, interval : Option<f32>) -> Option<bool> {
        if event_list.len() > 1 {
            for event in event_list {
                self.execute_events(&[*event], &updated_time, None)?;
            }
        }
        else if event_list.len() == 1{
            let new_list = self[&event_list[0]].execute(&updated_time, interval);
            self.execute_events(&new_list, &updated_time, None);
            self.happened_events_counter_[&event_list[0]].fetch_add(1, Ordering::Relaxed);
            //let mut happened_events = self.happened_events_counter_.lock().unwrap();
            //*(happened_events.get_mut(&event_list[0]).unwrap()) += 1;
        }
        else {

        }
        Some(true)
    }
}

impl<'a> Index<&usize> for EventContainer<'a> {
    type Output = Event<'a>;
    fn index(&self, event_id : &usize) -> &Self::Output {
        &self.events_list_[event_id]
    }
}

impl<'a> Index<&str> for EventContainer<'a> {
    type Output = Event<'a>;
    fn index(&self, event_name : &str) -> &Self::Output {
        let id = self.events_list_.get_id(event_name).unwrap();
        &self.events_list_[&id]
    }
}