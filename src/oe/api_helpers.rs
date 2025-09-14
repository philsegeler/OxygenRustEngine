use std::time;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use rayon::prelude::*;

use super::dummy_structs::*;
//use task_manager::*;
use super::event_handler::*;
use super::event::{EventFuncTrait, EventDataTrait};
use super::task::{TaskFuncTrait, TaskDataTrait};
use super::winsys_sdl2::*;
use super::natrium::renderer_compat::*;

type TaskEnum = super::task::TaskEnum;

use super::global_variables::*;

use super::carbon::interpreter::interpret_file;

pub fn init(x: u32, y: u32, title: &str) -> bool {
    let init_info = WinsysInitInfo {
        requested_backend: WinsysBackend::Angle,
    };
    let update_info = WinsysUpdateInfo {
        res_x: x,
        res_y: y,
        title: title.to_string(),
        use_fullscreen: false,
        res_changed : true,
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

    let renderer_compat = RendererCompat::new();
    {
        let mut global_renderer =OE_RENDERER_.lock().unwrap();
        global_renderer.replace(Box::new(renderer_compat));
    }
    if OE_USE_MULTIPLE_THREADS_{

        let start_cond = Arc::clone(&OE_START_CONDITION_);
        let end_cond = Arc::clone(&OE_END_CONDITION_);


        *OE_THREAD_HANDLE_.write().unwrap() = Some(thread::spawn(move ||{
            let mut count = 0;
            while !(OE_DONE_.load(Ordering::Relaxed)) {
                start_cond.update();
                update_tasks();
                update_objects();
                end_cond.update();
                update_scenegraph();
                update_events();
                //TODO: UPDATE RENDERER DATA
                count += 1;
               
            }
            println!("Update Thread invocations: {:?}", count);
        }));
    }

    *OE_WINSYS_INIT_INFO_.lock().unwrap() = Some(init_info);
    *OE_WINSYS_UPDATE_INFO_.lock().unwrap() = Some(update_info);
    *OE_RENDERER_UPDATE_INFO_.lock().unwrap() = Some(Default::default());
    true
}

pub fn set_title(name : &str){
    let mut winsys_update_info = OE_WINSYS_UPDATE_INFO_.lock().unwrap();
    winsys_update_info.as_mut().unwrap().title = name.into();
}

fn update_objects(){
    let scenegraph = OE_SCENEGRAPH_.lock().unwrap();
    for id in scenegraph.get_object_ids(){
        let object_mutexed = scenegraph.get_object(id);
        let mut object = object_mutexed.lock().unwrap();
        object.0.update();
    }
}

fn update_scenegraph(){
    let mut update_info_mutex = OE_RENDERER_UPDATE_INFO_.lock().unwrap();
    let update_info = (*update_info_mutex).unwrap();
    update_info_mutex.as_mut().unwrap().restart_renderer = false;
    drop(update_info_mutex);

    let mut scenegraph = OE_SCENEGRAPH_.lock().unwrap();
    let (changed_elems , events)= scenegraph.update(update_info.restart_renderer);
    drop(scenegraph);

    let winsys_output_mutex = OE_WINSYS_OUTPUT_INFO_.lock().unwrap();
    let winsys_output = 
    winsys_output_mutex.clone().unwrap();
    drop(winsys_output_mutex);

    let mut renderer = OE_RENDERER_.lock().unwrap();
    renderer.as_mut().unwrap().update_data(changed_elems, update_info, winsys_output);
    drop(renderer);

    for event in &events{
        super::broadcast_event_by_id(*event);
    }
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
        //let after = time::Instant::now();

        //println!("events: {:?}", (after-before).as_nanos()/1000);
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

pub fn step() -> bool {
    if !OE_USE_MULTIPLE_THREADS_ {
        update_tasks();
        update_objects();
    }
    else {
        OE_START_CONDITION_.update();
    }
    // renderer update
    {
        let mut renderer = OE_RENDERER_.lock().unwrap();
        renderer.as_mut().unwrap().update_single_thread();
    }

    // winsys update
    {
        let mut update_info = OE_WINSYS_UPDATE_INFO_.lock().unwrap();
        let output = OE_WINSYS_.with_borrow_mut(|winsys|winsys.update_window(update_info.clone().unwrap()));
        *update_info = Some(output.update_info.clone());
        *OE_WINSYS_OUTPUT_INFO_.lock().unwrap() = Some(output.clone());
    }

    let mut unsync_threads = OE_UNSYNC_THREADS_.lock().unwrap();
    for mut thread in std::mem::take(&mut *unsync_threads){
        thread.1 = thread.0.is_finished();
        if thread.1 {
            thread.0.join().unwrap();
        }
        else{
            unsync_threads.push(thread);
        }
    }
    drop(unsync_threads);

    //update winsys events
    OE_WINSYS_.with_borrow_mut(|winsys|winsys.update_events_single_thread());

    if OE_USE_MULTIPLE_THREADS_{
        OE_END_CONDITION_.update();
    }
    else {
        update_scenegraph();
        update_events();
    }
    let done = OE_WINSYS_.with_borrow(|winsys|winsys.is_done());
    done || OE_DONE_.load(Ordering::Relaxed)
}

pub  fn start() -> bool{
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

pub fn create_task_thread() -> usize {
    let mut task_manager_list = OE_TASK_MANAGERS_.lock().unwrap();
    task_manager_list.push(Default::default());
    task_manager_list.len()-1
}

pub fn add_task_func_data(thread_id : &usize, task_name : &str, func : impl TaskFuncTrait + 'static, data : Box<dyn TaskDataTrait>, type_in : TaskEnum, interval : Option<f32>) -> usize {
    let task_managers = OE_TASK_MANAGERS_.lock().unwrap();
    let mut task_manager = task_managers[*thread_id].write().unwrap();
    let task_id = task_manager.create_task(task_name, type_in);
    task_manager.set_task_func(&task_id, func, data);
    task_manager.set_task_interval(&task_id, interval);
    task_id
}

pub fn set_event_func_data(event_name : &str, func : impl EventFuncTrait + 'static, data : Box<dyn EventDataTrait>) -> bool {
    let mut event_handler = OE_EVENT_HANDLER_.write().unwrap();
    let event_id = event_handler.as_ref().unwrap().get_event_id(event_name).unwrap();
    let output = event_handler.as_mut().unwrap().set_event_func(&event_id, func, data).unwrap_or(false);
    output
}

pub fn load_world_func_data(filename : &str, func : impl EventFuncTrait + 'static, data : Box<dyn EventDataTrait>) -> bool{
    let filename_owned = filename.to_owned();
    let mut event_handler = OE_EVENT_HANDLER_.write().unwrap();
    let event_id = event_handler.as_mut().unwrap().create_load_event(filename);
    let output = event_handler.as_mut().unwrap().set_event_func(&event_id, func, data).unwrap_or(false);
    drop(event_handler);
    let handle = thread::spawn(move ||{
            let new_data = interpret_file(&filename_owned);
            let mut scenegraph = OE_SCENEGRAPH_.lock().unwrap();
            scenegraph.add_interpreted(new_data, event_id);
            println!("[UNSYNC THREAD] Loaded world from \"{:?}\"", filename_owned);
    });
    let mut threadhandles = OE_UNSYNC_THREADS_.lock().unwrap();
    threadhandles.push((handle, false));
    output
}