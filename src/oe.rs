mod base_traits;
mod dummy_structs;
mod event_handler;
mod task_manager;
mod winsys_sdl2;
mod event_container;
mod global_variables;
mod task_container;
mod event;
mod task;
mod api_helpers;
pub mod types;
pub mod math;
pub mod carbon;
pub mod natrium;

use event::{EventFuncTrait, EventFuncTraitWithoutArgs, EventDataTrait};
use task::{TaskFuncTrait, TaskFuncTraitWithoutArgs, TaskDataTrait};
use global_variables::*;

pub type EventInfo = event::EventInfo;
pub type EventEnum = event::EventEnum;
pub type TaskInfo = task::TaskInfo;
pub type TaskEnum = task::TaskEnum;
pub type TaskOutput = task::TaskOutput;

/// basic initialisation
pub fn init(x: u32, y: u32, title: &str) -> bool {
    api_helpers::init(x, y, title)
}

#[no_mangle]
pub extern "C" fn step() -> bool {
    api_helpers::step()
}

#[no_mangle]
pub extern "C" fn start() -> bool{
    api_helpers::start()
}

pub fn set_title(name : &str){
    api_helpers::set_title(name);
}

/// task handling
#[no_mangle]
pub extern "C" fn create_task_thread() -> usize {
    api_helpers::create_task_thread()
}


pub fn add_task_func(thread_id : &usize, task_name : &str, func : impl TaskFuncTraitWithoutArgs + 'static, type_in : TaskEnum, interval : Option<f32>) -> usize {
    api_helpers::add_task_func_data(thread_id, task_name, move |info, _|{func(info)}, Box::new(0), type_in, interval)
}

pub fn add_task_func_data(thread_id : &usize, task_name : &str, func : impl TaskFuncTrait + 'static, data : Box<dyn TaskDataTrait>, type_in : TaskEnum, interval : Option<f32>) -> usize {
    api_helpers::add_task_func_data(thread_id, task_name, func, data, type_in, interval)
}
pub fn set_task_data_by_id(thread_id : &usize, task_id : &usize, data : Box<dyn TaskDataTrait>){
    let task_managers = OE_TASK_MANAGERS_.lock().unwrap();
    let task_manager = task_managers[*thread_id].write().unwrap();
    task_manager.set_task_data(task_id, data);
}

pub fn set_task_data(thread_id : &usize, task_name : &str, data : Box<dyn TaskDataTrait>) -> Option<bool>{
    let task_managers = OE_TASK_MANAGERS_.lock().unwrap();
    let task_manager = task_managers[*thread_id].write().unwrap();
    let task_id = task_manager.get_task_id(task_name)?;
    task_manager.set_task_data(&task_id, data);
    Some(true)
}

pub fn get_task_name(thread_id : &usize, id : &usize) -> String {
    let task_managers = OE_TASK_MANAGERS_.lock().unwrap();
    let task_manager = task_managers[*thread_id].read().unwrap();
    let output = task_manager.get_task_name(id).unwrap().to_string();
    output
}
pub fn get_task(thread_id : &usize, event_name : &str) -> usize {
    let task_managers = OE_TASK_MANAGERS_.lock().unwrap();
    let task_manager = task_managers[*thread_id].read().unwrap();
    let output = task_manager.get_task_id(event_name).unwrap_or(0);
    output
}

/// event handling
pub fn create_user_event(event_name : &str) -> usize {
    let mut event_handler = OE_EVENT_HANDLER_.write().unwrap();
    let output = event_handler.as_mut().unwrap().create_user_event(event_name);
    output
}
pub fn get_user_event(event_name : &str) -> usize {
    let event_handler = OE_EVENT_HANDLER_.read().unwrap();
    let output = event_handler.as_ref().unwrap().get_user_event_id(event_name).unwrap_or(0);
    output
}
pub fn get_user_events() -> Vec<usize> {
    let event_handler = OE_EVENT_HANDLER_.read().unwrap();
    let output = event_handler.as_ref().unwrap().get_user_events();
    output
}
pub fn get_event_name(id : &usize) -> String {
    let event_handler = OE_EVENT_HANDLER_.read().unwrap();
    let output = event_handler.as_ref().unwrap().get_event_name(id).unwrap().to_string();
    output
}

#[no_mangle]
pub extern "C" fn broadcast_event_by_id(event_id : usize) -> bool {
    let event_handler = OE_EVENT_HANDLER_.read().unwrap();
    let output = event_handler.as_ref().unwrap().broadcast_event(&event_id).unwrap_or(false);
    output
} 

pub fn broadcast_event(event_name : &str) -> bool {
    let event_handler = OE_EVENT_HANDLER_.read().unwrap();
    let event_id = event_handler.as_ref().unwrap().get_event_id(event_name).unwrap();
    let output = event_handler.as_ref().unwrap().broadcast_event(&event_id).unwrap_or(false);
    output
} 

#[no_mangle]
pub extern "C" fn repeat_event_by_id(event_id : usize) -> bool {
    let event_handler = OE_EVENT_HANDLER_.read().unwrap();
    let output = event_handler.as_ref().unwrap().repeat_event(&event_id).unwrap_or(false);
    output
} 

pub fn repeat_event(event_name : &str) -> bool {
    let event_handler = OE_EVENT_HANDLER_.read().unwrap();
    let event_id = event_handler.as_ref().unwrap().get_event_id(event_name).unwrap();
    let output = event_handler.as_ref().unwrap().repeat_event(&event_id).unwrap_or(false);
    output
} 

#[no_mangle]
pub extern "C" fn repeat_timed_event_by_id(event_id : &usize,  interval : f32) -> bool {
    let event_handler = OE_EVENT_HANDLER_.read().unwrap();
    let output = event_handler.as_ref().unwrap().repeat_timed_event(event_id, interval).unwrap_or(false);
    output
} 

pub fn repeat_timed_event(event_name : &str, interval : f32) -> bool {
    let event_handler = OE_EVENT_HANDLER_.read().unwrap();
    let event_id = event_handler.as_ref().unwrap().get_event_id(event_name).unwrap();
    let output = event_handler.as_ref().unwrap().repeat_timed_event(&event_id, interval).unwrap_or(false);
    output
} 

#[no_mangle]
pub extern "C" fn derepeat_event_by_id(event_id : &usize) -> bool {
    let event_handler = OE_EVENT_HANDLER_.read().unwrap();
    let output = event_handler.as_ref().unwrap().derepeat_event(event_id).unwrap_or(false);
    output
} 

pub fn derepeat_event(event_name : &str) -> bool {
    let event_handler = OE_EVENT_HANDLER_.read().unwrap();
    let event_id = event_handler.as_ref().unwrap().get_event_id(event_name).unwrap();
    let output = event_handler.as_ref().unwrap().derepeat_event(&event_id).unwrap_or(false);
    output
} 

pub fn set_event_func_by_id(event_id : &usize, func : impl EventFuncTraitWithoutArgs + 'static) -> bool {
    let mut event_handler = OE_EVENT_HANDLER_.write().unwrap();
    let output = event_handler.as_mut().unwrap().set_event_func(event_id, move |info, _|{func(info)}, Box::new(0)).unwrap_or(false);
    output
}

pub fn set_event_func(event_name : &str, func : impl EventFuncTraitWithoutArgs + 'static) -> bool {
    api_helpers::set_event_func_data(event_name, move |info: &event::EventInfo, _|{func(info)},Box::new(0))
}

pub fn set_event_func_data_by_id(event_id : &usize, func : impl EventFuncTrait + 'static, data : Box<dyn EventDataTrait>) -> bool {
    let mut event_handler = OE_EVENT_HANDLER_.write().unwrap();
    let output = event_handler.as_mut().unwrap().set_event_func(event_id, func, data).unwrap_or(false);
    output
}

pub fn set_event_func_data(event_name : &str, func : impl EventFuncTrait + 'static, data : Box<dyn EventDataTrait>) -> bool {
    api_helpers::set_event_func_data(event_name, func, data)
}

pub fn set_event_data(event_name : &str, data : Box<dyn EventDataTrait>) -> bool {
    let mut event_handler = OE_EVENT_HANDLER_.write().unwrap();
    let event_id = event_handler.as_ref().unwrap().get_event_id(event_name).unwrap();
    let output = event_handler.as_mut().unwrap().set_event_data(&event_id, data).unwrap_or(false);
    output
}
pub fn set_event_data_by_id(event_id : &usize, data : Box<dyn EventDataTrait>) -> bool {
    let mut event_handler = OE_EVENT_HANDLER_.write().unwrap();
    let output = event_handler.as_mut().unwrap().set_event_data(event_id, data).unwrap_or(false);
    output
}

pub mod keyboard{
    use super::OE_EVENT_HANDLER_;
    pub fn get_event(event_name : &str) -> usize {
        let event_handler = OE_EVENT_HANDLER_.read().unwrap();
        let output = event_handler.as_ref().unwrap().get_keyboard_event_id(event_name).unwrap_or(0);
        output
    }
    pub fn get_events() -> Vec<usize> {
        let event_handler = OE_EVENT_HANDLER_.read().unwrap();
        let output = event_handler.as_ref().unwrap().get_keyboard_events();
        output
    }
}
pub mod mouse{
    use super::{OE_EVENT_HANDLER_, OE_WINSYS_UPDATE_INFO_};
    pub fn get_event(event_name : &str) -> usize {
        let event_handler = OE_EVENT_HANDLER_.read().unwrap();
        let output = event_handler.as_ref().unwrap().get_mouse_event_id(event_name).unwrap_or(0);
        output
    }
    pub fn get_events() -> Vec<usize> {
        let event_handler = OE_EVENT_HANDLER_.read().unwrap();
        let output = event_handler.as_ref().unwrap().get_mouse_events();
        output
    }
    #[no_mangle]
    pub extern "C" fn lock() -> bool {
        {
            OE_WINSYS_UPDATE_INFO_.lock().unwrap().as_mut().unwrap().mouse_locked = true;
        }
        let event_handler = OE_EVENT_HANDLER_.read().unwrap();
        let event_id = event_handler.as_ref().unwrap().get_mouse_event_id("lock").unwrap();
        let output = event_handler.as_ref().unwrap().broadcast_event(&event_id).unwrap_or(false);
        output
    }
    #[no_mangle]
    pub extern "C" fn unlock() -> bool {
        {
            OE_WINSYS_UPDATE_INFO_.lock().unwrap().as_mut().unwrap().mouse_locked = false;
        }
        let event_handler = OE_EVENT_HANDLER_.read().unwrap();
        let event_id = event_handler.as_ref().unwrap().get_mouse_event_id("unlock").unwrap();
        let output = event_handler.as_ref().unwrap().broadcast_event(&event_id).unwrap_or(false);
        output
    }
    #[no_mangle]
    pub extern "C" fn set_lock(value : bool) -> bool{
        {
            OE_WINSYS_UPDATE_INFO_.lock().unwrap().as_mut().unwrap().mouse_locked = value;
        }
        let event_handler = OE_EVENT_HANDLER_.read().unwrap();
        let event_name = if value {"lock"} else {"unlock"};
        let event_id = event_handler.as_ref().unwrap().get_mouse_event_id(event_name).unwrap();
        let output = event_handler.as_ref().unwrap().broadcast_event(&event_id).unwrap_or(false);
        output
    }
    #[no_mangle]
    pub extern "C" fn toggle_lock() -> bool{
        let value;
        {
            let mut update_info = OE_WINSYS_UPDATE_INFO_.lock().unwrap();
            update_info.as_mut().unwrap().mouse_locked = !update_info.as_ref().unwrap().mouse_locked;
            value = update_info.as_ref().unwrap().mouse_locked;
        }
        let event_handler = OE_EVENT_HANDLER_.read().unwrap();
        let event_name = if value {"lock"} else {"unlock"};
        let event_id = event_handler.as_ref().unwrap().get_mouse_event_id(event_name).unwrap();
        let output = event_handler.as_ref().unwrap().broadcast_event(&event_id).unwrap_or(false);
        output
    }
}

/// scenegraph load elements with associated event
pub fn load_world_func(filename : &str, func : impl EventFuncTraitWithoutArgs + 'static) -> bool{
    api_helpers::load_world_func_data(filename, move |info: &event::EventInfo, _|{func(info)}, Box::new(0))
}
pub fn load_world_func_data(filename : &str, func : impl EventFuncTrait + 'static, data : Box<dyn EventDataTrait>) -> bool{
    api_helpers::load_world_func_data(filename, func, data)
}