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
pub mod types;
pub mod math;
pub mod carbon;

use dummy_structs::*;
use task::TaskDataTrait;
//use task_manager::*;
use winsys_sdl2::*;
use event_handler::*;
use event::{EventFuncTrait, EventFuncTraitWithoutArgs, EventDataTrait};
use task::{TaskFuncTrait, TaskFuncTraitWithoutArgs};
use global_variables::*;
use rayon::prelude::*;

pub type EventInfo = event::EventInfo;
pub type EventEnum = event::EventEnum;
pub type TaskInfo = task::TaskInfo;
pub type TaskEnum = task::TaskEnum;
pub type TaskOutput = task::TaskOutput;

use std::time;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::Ordering;

/// basic initialisation
pub fn init(x: u32, y: u32, title: &str) -> bool {
    let init_info = WinsysInitInfo {
        requested_backend: WinsysBackend::Angle,
    };
    let update_info = WinsysUpdateInfo {
        res_x: x,
        res_y: y,
        title: title.to_string(),
        use_fullscreen: false,

        vsync: true,
        mouse_locked: false,
    };

    let event_handler : EventHandler = Default::default();
    {
        OE_EVENT_HANDLER_.write().unwrap().replace(event_handler);
    }

    create_task_thread();

    let winsys = WinsysSdl2::new(&init_info, &update_info, Arc::clone(&OE_EVENT_HANDLER_));
    
    OE_WINSYS_.set(Box::new(winsys));

    if OE_USE_MULTIPLE_THREADS_{

        let start_cond = Arc::clone(&OE_START_CONDITION_);
        let end_cond = Arc::clone(&OE_END_CONDITION_);


        *OE_THREAD_HANDLE_.write().unwrap() = Some(thread::spawn(move ||{
            let mut count = 0;
            while !(OE_DONE_.load(Ordering::Relaxed)) {
                start_cond.update();
                update_tasks();
                end_cond.update();
                update_events();
                count += 1;
               
            }
             println!("Update Thread invocations: {:?}", count);
        }));
    }

    *OE_WINSYS_INIT_INFO_.lock().unwrap() = Some(init_info);
    *OE_WINSYS_UPDATE_INFO_.lock().unwrap() = Some(update_info);
    true
}

fn update_events(){

    let before;
    before = time::Instant::now();
    if OE_USE_MULTIPLE_THREADS_ {
        let event_container;
        {
            let event_handler = OE_EVENT_HANDLER_.read().unwrap();

            event_container = event_handler.as_ref().unwrap().consume_events();
        //event_handler.as_ref().unwrap().handle_all_events(&before).unwrap();
        }
        event_container.handle_all_events(&before);
    }
    else {
        let event_container;
        {
            let event_handler = OE_EVENT_HANDLER_.read().unwrap();
            event_container = event_handler.as_ref().unwrap().consume_events_as_is();
        }
        event_container.handle_all_events(&before);
    }
    {

        let mut event_handler = OE_EVENT_HANDLER_.write().unwrap();
        event_handler.as_mut().unwrap().update_event_timestamps(&before);
        let after = time::Instant::now();

        println!("events: {:?}", (after-before).as_nanos()/1000);
    }
}
fn update_tasks(){
    let mut task_manager_list = vec![];
    {
        let lista = OE_TASK_MANAGERS_.lock().unwrap();
        for task_mgr in lista.iter(){
            task_manager_list.push(Arc::clone(&task_mgr));
        }
    }
    let before;
    before = time::Instant::now();
    task_manager_list.par_iter().enumerate().for_each(|(thread_id, task_manager)|{
        let to_be_removed;
        if OE_USE_MULTIPLE_THREADS_ {
            let task_container;
            {
                let taskmgr = task_manager.read().unwrap();
                task_container = taskmgr.consume_tasks();
            }
            to_be_removed = task_container.run_tasks(&thread_id, &before);
        }
        else {
            let task_container;
            {
                let taskmgr = task_manager.read().unwrap();
                task_container = taskmgr.consume_tasks_as_is();
            }
            to_be_removed = task_container.run_tasks(&thread_id, &before);
        }
        {
            let mut taskmgr = task_manager.write().unwrap();
            taskmgr.update_task_timestamps(&before);
            taskmgr.remove_tasks(to_be_removed);
        }
    });
    
}

#[no_mangle]
pub extern "C" fn step() -> bool {
    if !OE_USE_MULTIPLE_THREADS_ {
        update_tasks();
    }
    else {
        OE_START_CONDITION_.update();
    }

    /*let winsys;
    unsafe {
        winsys = OE_WINSYS_.borrow_mut().unwrap();
    }*/
    // renderer update
    
    let output;
    {
        let mut update_info = OE_WINSYS_UPDATE_INFO_.lock().unwrap();
        output = OE_WINSYS_.with_borrow_mut(|winsys|winsys.update_window(update_info.clone().unwrap()));
        *update_info = Some(output.update_info.clone());
        *OE_WINSYS_OUTPUT_INFO_.lock().unwrap() = Some(output);
    }

    //pending renderer data
    //pending interpreter data

    OE_WINSYS_.with_borrow_mut(|winsys|winsys.update_events_single_thread());

    if OE_USE_MULTIPLE_THREADS_{
        OE_END_CONDITION_.update();
    }
    else {
        update_events();
    }
    let done = OE_WINSYS_.with_borrow(|winsys|winsys.is_done());
    done || OE_DONE_.load(Ordering::Relaxed)
}

#[no_mangle]
pub extern "C" fn start() -> bool{
    let output = true;

    let mut count = 0;

    #[cfg(not(target_os = "emscripten"))]
    while !(OE_DONE_.load(Ordering::Relaxed)) {
        let output = step();
        count+=1;
        
        OE_DONE_.store(output, Ordering::Relaxed);
    }
    println!("Main Thread invocations: {:?}", count);
    
    if OE_USE_MULTIPLE_THREADS_{
        OE_START_CONDITION_.release();
        OE_END_CONDITION_.release();
        let mut thread_handle = OE_THREAD_HANDLE_.write().unwrap();
        thread_handle.take().unwrap().join().unwrap();
    }
    #[cfg(target_os = "emscripten")]
    use emscripten::emscripten;
    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(||{step()});
    output
}

/// task handling
#[no_mangle]
pub extern "C" fn create_task_thread() -> usize {
    let mut task_manager_list = OE_TASK_MANAGERS_.lock().unwrap();
    task_manager_list.push(Default::default());
    task_manager_list.len()-1
}


pub fn add_task_func(thread_id : &usize, task_name : &str, func : impl TaskFuncTraitWithoutArgs + 'static, type_in : TaskEnum, interval : Option<f32>) -> usize {
    let task_managers = OE_TASK_MANAGERS_.lock().unwrap();
    let mut task_manager = task_managers[*thread_id].write().unwrap();
    let task_id = task_manager.create_task(task_name, type_in);
    task_manager.set_task_func(&task_id, move |info, _|{func(info)}, Box::new(0));
    task_manager.set_task_interval(&task_id, interval);
    task_id
}

pub fn add_task_func_data(thread_id : &usize, task_name : &str, func : impl TaskFuncTrait + 'static, data : Box<dyn TaskDataTrait>, type_in : TaskEnum, interval : Option<f32>) -> usize {
    let task_managers = OE_TASK_MANAGERS_.lock().unwrap();
    let mut task_manager = task_managers[*thread_id].write().unwrap();
    let task_id = task_manager.create_task(task_name, type_in);
    task_manager.set_task_func(&task_id, func, data);
    task_manager.set_task_interval(&task_id, interval);
    task_id
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
    let mut event_handler = OE_EVENT_HANDLER_.write().unwrap();
    let event_id = event_handler.as_ref().unwrap().get_event_id(event_name).unwrap();
    let output = event_handler.as_mut().unwrap().set_event_func(&event_id, move |info, _|{func(info)}, Box::new(0)).unwrap_or(false);
    output
}

pub fn set_event_func_data_by_id(event_id : &usize, func : impl EventFuncTrait + 'static, data : Box<dyn EventDataTrait>) -> bool {
    let mut event_handler = OE_EVENT_HANDLER_.write().unwrap();
    let output = event_handler.as_mut().unwrap().set_event_func(event_id, func, data).unwrap_or(false);
    output
}

pub fn set_event_func_data(event_name : &str, func : impl EventFuncTrait + 'static, data : Box<dyn EventDataTrait>) -> bool {
    let mut event_handler = OE_EVENT_HANDLER_.write().unwrap();
    let event_id = event_handler.as_ref().unwrap().get_event_id(event_name).unwrap();
    let output = event_handler.as_mut().unwrap().set_event_func(&event_id, func, data).unwrap_or(false);
    output
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
        /*let is_locked;
        {
            is_locked = OE_WINSYS_UPDATE_INFO_.lock().unwrap().as_ref().unwrap().mouse_locked
        };
        if is_locked {
            unlock()
        }
        else {
            lock()
        }*/
    }
}