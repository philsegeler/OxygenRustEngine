//extern crate sdl2;
//extern crate gl;
#[allow(dead_code)]
pub mod oe;
//use OE as oe;
extern crate rand;
use std::sync::{Arc, Mutex};
use std::sync::LazyLock;
use nohash_hasher::IntMap;
//use rayon::prelude::*;
//use std::collections::HashMap;
#[cfg(target_os = "emscripten")]
pub mod emscripten;

// TEST PARAMETERS
static TOTAL_EVENTS : usize = 42000;
static ACTIVE_EVENTS : usize = 1000;
static OVERHEAD_MIN : usize = 4;
static OVERHEAD_MAX : usize = 10;

fn task_test_fn(_info: &oe::TaskInfo) -> oe::TaskOutput{
    //let task_name = oe::get_task_name(info.thread_id(), info.id());
    //println!("{:?}", task_name);
    //println!("{:?}", info);
    oe::TaskOutput::Keep
}

fn run_event(info : &oe::EventInfo) -> Vec<usize>{
    println!("{}", oe::get_event_name(info.id()));
    //println!("{:?}", info);
    vec![]
}

static EVENTS_LIST : LazyLock<Arc<Mutex<Vec<usize>>>> = LazyLock::new(||Default::default());
static EVENTS_MAP : LazyLock<Arc<Mutex<IntMap<usize, isize>>>> = LazyLock::new(||Default::default());

fn event_func(index : usize) -> Vec<usize> {
    let mut to_add  = 1;
    for _ in 0..(rand::random::<usize>() % OVERHEAD_MAX + OVERHEAD_MIN){
        to_add += rand::random::<isize>() % 3-2;
    }
    *EVENTS_MAP.lock().unwrap().get_mut(&index).unwrap() += to_add;
    vec![]
}

fn broadcast_all_events(_ : &oe::TaskInfo) -> oe::TaskOutput {
    //println!("Broadcasting events");

    let eh = EVENTS_LIST.lock().unwrap();
    eh[0..ACTIVE_EVENTS].iter().for_each(|i|{
        oe::broadcast_event_by_id(*i);
    });
    oe::broadcast_event_by_id(eh[rand::random::<usize>() % (eh.len() as usize)]);
    oe::TaskOutput::Keep
}


fn create_events(_ : &oe::TaskInfo) -> oe::TaskOutput {
    //println!("Creating Events TASK 1");
    let broadcast_event = oe::create_user_event("launch broadcasting");
    oe::set_event_func_by_id(&broadcast_event, |_| {oe::add_task_func(&0,"event broadcast", &broadcast_all_events, oe::TaskEnum::Repeat, None);vec![]});
    for _ in 0..TOTAL_EVENTS {
        let event_id = oe::create_user_event(&("test_event".to_owned() + stringify!(i)));
    
        oe::set_event_func_by_id(&event_id, |&info|event_func(*info.id()));
        EVENTS_MAP.lock().unwrap().insert(event_id, 0);

        EVENTS_LIST.lock().unwrap().push(event_id);

    }
    oe::broadcast_event_by_id(broadcast_event);
    //println!("Finished creating tasks");
    oe::TaskOutput::Drop
}


fn main() {

    oe::init(640, 480, "Some Rust Demo 2");
    
    oe::set_event_func("keyboard-Space+", |info| {oe::set_title(&("Space key pressed: ".to_owned() + &info.invocations().to_string()));oe::mouse::toggle_lock();vec![]});
    
    let event_id = oe::create_user_event("repeat after 5 secs");
    oe::set_event_func_by_id(&event_id, |info|{run_event(info)});
    //oe::repeat_timed_event_by_id(&event_id, 5.0);
    
    oe::add_task_func(&0,"add everything", &create_events, oe::TaskEnum::Once, None);
    //oe::repeat_timed_event_by_id(&event_id, 5.0);
    oe::add_task_func(&0, "test_task", &task_test_fn, oe::TaskEnum::Repeat, Some(2.0));
    oe::load_world_func("OE_VerySimple.csl", &run_event);
    oe::load_world_func("OE_Demo_50MB.csl", &run_event);

    oe::start();
    //let x = rand::random::<usize>() % (TOTAL_EVENTS-1 as usize);
    //println!("{}", x);
    //let a = EVENTS_LIST.lock().unwrap()[x];
    //println!("{:?}", EVENTS_MAP.lock().unwrap()[&(a)]);
    //println!("{:?}", a);

    use std::mem::size_of;
    println!("{:?} {:?} {:?}", size_of::<Box<[u32]>>(), size_of::<i64>(), size_of::<u64>());
}
