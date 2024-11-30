use std::sync::{Arc, Mutex};
use std::time;
use std::fmt;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::any::Any;

#[derive(Clone, Debug, Copy)]
#[repr(C)]
pub struct EventInfo{
    id_ : usize,
    invocations_ : usize,
    delta_ : f64,
    type_ : EventEnum,
}
impl EventInfo {
    pub fn id (&self) -> &usize {
        &self.id_
    }
    pub fn invocations(&self) -> &usize {
        &self.invocations_
    }
    pub fn deltatime(&self) -> &f64 {
        &self.delta_
    }
    pub fn get_type(&self) -> &EventEnum {
        &self.type_
    }
}

pub trait EventDataTrait : Any + Send {}
impl<T> EventDataTrait for T where T : Any + Send{}
pub trait EventFuncTrait : Fn(&EventInfo, &Box<dyn EventDataTrait>) -> Vec<usize> + Send + Sync {}
impl<T> EventFuncTrait for T where T : Fn(&EventInfo,  &Box<dyn EventDataTrait>) -> Vec<usize> + Send + Sync {}
pub trait EventFuncTraitWithoutArgs : Fn(&EventInfo) -> Vec<usize> + Send + Sync {}
impl<T> EventFuncTraitWithoutArgs for T where T : Fn(&EventInfo) -> Vec<usize> + Send + Sync {}



#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum EventEnum {
    User,
    Keyboard,
    Mouse,
    Internal,
}

pub struct EventMutexedProperties<'a> {
    func_ : Box<dyn EventFuncTrait + 'a>,
    data_ : Box<dyn EventDataTrait>,
}

pub struct EventArcedProperties<'a>{
    invocations_ : AtomicUsize,
    executed : AtomicBool,
    mutexed_ : Mutex<EventMutexedProperties<'a>>
}

// EVENT
#[derive(Clone)]
pub struct Event<'a> {
    id_ : usize,
    pub active : bool,
    data : Arc<EventArcedProperties<'a>>,
    timestamp_ : time::Instant,
    type_ : EventEnum,
}

impl std::fmt::Debug for Event<'_>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = &self.data;
        f.debug_struct("Event")
         .field("id_", &self.id_)
         .field("active", &self.active)
         .field("type_", &self.type_)
         .field("invocations", &data.invocations_)
         .field("timestamp", &{time::Instant::now() - self.timestamp_}.as_secs_f64())
         .finish()
    }
} 

impl<'a> Event<'a> {
    pub fn new( func :Option<Box<dyn EventFuncTrait + 'a>>, event_type : EventEnum) -> Event{
        static ID_COUNT : AtomicUsize = AtomicUsize::new(1);
        //static EVENT_TIMESTAMP : LazyLock<time::Instant> = LazyLock::new(||time::Instant::now());
        Event{
            id_ : ID_COUNT.fetch_add(1, Ordering::Relaxed),
            active : true,
            type_ : event_type,
            timestamp_ : time::Instant::now(),
            data : Arc::new(EventArcedProperties{
                invocations_ : AtomicUsize::new(0),
                executed : AtomicBool::new(false),
                mutexed_ : Mutex::new(EventMutexedProperties{
                    func_ : func.unwrap_or(Box::new(|_, _| {vec![]})),
                    data_ : Box::new(0)
                })
            }),
            }
    }

    pub fn set_func(&self, func : Box<dyn EventFuncTrait + 'a>, data : Box<dyn EventDataTrait>){
        let mut mutexed_data = self.data.mutexed_.lock().unwrap();
        mutexed_data.func_ = func;
        mutexed_data.data_ = data;
        //self.data.func_ = Arc::new(func);
    }
    pub fn set_func_data(&self, data : Box<dyn EventDataTrait>){
        let mut mutexed_data = self.data.mutexed_.lock().unwrap();
        mutexed_data.data_ = data;
    }

    pub fn set_type(&mut self, event_type : EventEnum){
        self.type_ = event_type;
    }

    pub fn get_type(&self) -> EventEnum {
        self.type_
    }

    pub fn id(&self) -> usize {
        self.id_
    }
    pub fn invocations(&self) -> usize{
        let data = &self.data;
        data.invocations_.load(Ordering::Relaxed)
    }
    pub fn update(&mut self, updated_time : &time::Instant){
        let data = &self.data;
        if data.executed.swap(false, Ordering::Relaxed){
            self.timestamp_ = *updated_time;
        }
    }

    pub fn execute(&self, updated_time : &time::Instant, interval : Option<f32>) -> Vec<usize>{
        if self.active {
            //println!("{:?}", &self);
            //let (delta, invoc) = self.update_elapsed(updated_time);
            let data = &self.data;
            let cur_time = self.timestamp_;
            let (delta, invoc) = ({*updated_time - cur_time}.as_secs_f64(), data.invocations_.load(Ordering::Relaxed));
            //let func = &(data.func);
            match interval {
                    Some(i) => if (delta - i as f64) > 1e-8 {
                        data.invocations_.fetch_add(1, Ordering::Relaxed);
                        data.executed.store(true, Ordering::Relaxed);
                        //data.timestamp_ = *updated_time;
                        let mutexed_data = data.mutexed_.lock().unwrap();
                        (mutexed_data.func_)(&EventInfo {id_:self.id_, invocations_ : invoc+1, delta_ : delta, type_:self.type_ }, &mutexed_data.data_)
                    }
                    else{
                        vec![]
                    }
                    None => {
                        data.invocations_.fetch_add(1, Ordering::Relaxed);
                        data.executed.store(true, Ordering::Relaxed);
                        //data.timestamp_ = *updated_time;
                        let mutexed_data = data.mutexed_.lock().unwrap();
                        (mutexed_data.func_)(&EventInfo {id_:self.id_, invocations_ : invoc+1, delta_ : delta, type_:self.type_ }, &mutexed_data.data_)
                    }
            }
        }
        else {
            vec![]
        }
    }
}