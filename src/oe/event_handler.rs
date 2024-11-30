use super::event_container::*;
//use std::sync::Mutex;

//use rayon::ThreadPoolBuilder;
use std::vec::Vec;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;


use std::time;
use crate::oe;
use super::event::*;

fn event_default_fn(info: &EventInfo, _ : &Box<dyn EventDataTrait>) -> Vec<usize>{
    let event_name = oe::get_event_name(info.id());
    if event_name != "mouse-motion" {
        println!("{:?}", event_name);
    }
    vec![]
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct MouseCoords{
    pub x : i32,
    pub y : i32,
}

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub enum ButtonStatus {
    JustPressed,
    Pressed,
    JustReleased,
    #[default]
    Released
}

#[derive(Default)]
pub struct EventHandler<'a>{
    events_: Arc<EventContainer<'a>>,
    mouse_pos_ : MouseCoords,
    mouse_delta_ : MouseCoords,
    mouse_moved_ : bool,
}

impl<'a> EventHandler<'a> {
    pub fn new() -> EventHandler<'a> {
        Default::default()
    }

    pub fn broadcast_event(&self, event_id : &usize) -> Option<bool>{
        self.events_.broadcast(event_id)
    }
    pub fn repeat_event(&self, event_id : &usize) -> Option<bool>{
        self.events_.repeat(event_id, None)
    }
    pub fn repeat_timed_event(&self, event_id : &usize, interval : f32) -> Option<bool>{
        self.events_.repeat(event_id, Some(interval))
    }
    pub fn derepeat_event(&self, event_id : &usize) -> Option<bool>{
        self.events_.derepeat(event_id)
    }

    pub fn create_user_event(&mut self, event_name : &str) -> usize {
        let id =Arc::get_mut(&mut self.events_).unwrap().insert(&("user-".to_string() + event_name), Box::new(&event_default_fn), EventEnum::User);
        let happened_events_counter = Arc::get_mut(&mut Arc::get_mut(&mut self.events_).unwrap().happened_events_counter_).unwrap();
        happened_events_counter.insert(id, AtomicU32::new(0));
        id
    }

    pub fn create_keyboard_event(&mut self, event_name : &str) -> usize {
        let id =Arc::get_mut(&mut self.events_).unwrap().insert(&("keyboard-".to_string() + event_name), Box::new(&event_default_fn), EventEnum::Keyboard);
        let happened_events_counter = Arc::get_mut(&mut Arc::get_mut(&mut self.events_).unwrap().happened_events_counter_).unwrap();
        happened_events_counter.insert(id, AtomicU32::new(0));
        id    
    }
    pub fn create_mouse_event(&mut self, event_name : &str) -> usize {
        let id = Arc::get_mut(&mut self.events_).unwrap().insert(&("mouse-".to_string() + event_name), Box::new(&event_default_fn), EventEnum::Mouse);
        let happened_events_counter = Arc::get_mut(&mut Arc::get_mut(&mut self.events_).unwrap().happened_events_counter_).unwrap();
        happened_events_counter.insert(id, AtomicU32::new(0));
        id
    }

    pub fn set_event_func(&self, event_id : &usize, func : impl EventFuncTrait + 'a, data : Box<dyn EventDataTrait>) -> Option<bool>{
        self.events_.set_func(event_id, Box::new(func), data)
    }
    pub fn set_event_data(&self, event_id : &usize, data : Box<dyn EventDataTrait>) -> Option<bool>{
        self.events_.set_func_data(event_id, data)
    }

    pub fn get_event_name(&self, event_id : &usize) -> Option<&str> {
        self.events_.get_name(event_id)
    }
    pub fn get_event_id(&self, event_name : &str) -> Option<usize> {
        self.events_.get_id(event_name)
    }

    pub fn get_user_event_id(&self, event_name : &str) -> Option<usize> {
        self.events_.get_id(&("user-".to_string() + event_name))
    }
    pub fn get_user_events(&self) -> Vec<usize> {
        let mut output : Vec<usize> = vec![];

        for event in self.events_.ids() {
            if self.events_[&event].get_type() == EventEnum::User{
                output.push(event);
            }
        }
        output
    }

    pub fn get_keyboard_event_id(&self, event_name : &str) -> Option<usize> {
        self.events_.get_id(&("keyboard-".to_string() + event_name))
    }
    pub fn get_keyboard_events(&self) -> Vec<usize> {
        let mut output : Vec<usize> = vec![];

        for event in self.events_.ids() {
            if self.events_[&event].get_type() == EventEnum::Keyboard{
                output.push(event);
            }
        }
        output
    }
    pub fn get_mouse_event_id(&self, event_name : &str) -> Option<usize> {
        self.events_.get_id(&("mouse-".to_string() + event_name))
    }
    pub fn get_mouse_events(&self) -> Vec<usize> {
        let mut output : Vec<usize> = vec![];

        for event in self.events_.ids() {
            if self.events_[&event].get_type() == EventEnum::Mouse{
                output.push(event);
            }
        }
        output
    }
    
    pub fn get_internal_event_id(&self, event_name : &str) -> Option<usize> {
        self.events_.get_id(&("internal-".to_string() + event_name))
    }
    pub fn get_internal_events(&self) -> Vec<usize> {
        let mut output : Vec<usize> = vec![];

        for event in self.events_.ids() {
            if self.events_[&event].get_type() == EventEnum::Internal{
                output.push(event);
            }
        }
        output
    }

    // internal functions 
    pub fn update_mouse_status(&mut self, pos: MouseCoords, delta : MouseCoords){
        self.mouse_pos_ = pos;
        self.mouse_delta_ = delta;
    }

    pub fn update_event_timestamps(&mut self, updated_time : &time::Instant){
        let event_map = Arc::get_mut(&mut self.events_).unwrap();
        for event_id in event_map.ids(){
            event_map.update_event(&event_id, updated_time);
        }

    }

    pub fn consume_events(&self) -> EventContainer<'a>{
        self.events_.consume()
    }
    pub fn consume_events_as_is(&self) -> Arc<EventContainer<'a>>{
        Arc::clone(&self.events_)
    }
    
}