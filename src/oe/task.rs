use std::sync::{Arc, Mutex};
use std::time;
use std::fmt;
use std::any::Any;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use core::cmp;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TaskEnum{
    Once,
    Repeat,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TaskOutput{
    Keep,
    Drop,
}

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct TaskInfo{
    id_ : usize,
    invocations_ : usize,
    delta_ : f64,
    type_ : TaskEnum,
    thread_id_ : usize,
}

impl TaskInfo {
    pub fn id (&self) -> &usize {
        &self.id_
    }
    pub fn invocations(&self) -> &usize {
        &self.invocations_
    }
    pub fn deltatime(&self) -> &f64 {
        &self.delta_
    }
    pub fn get_type(&self) -> &TaskEnum {
        &self.type_
    }
    pub fn thread_id(&self) -> &usize {
        &self.thread_id_
    }
}

pub trait TaskDataTrait : Any + Send {}
impl<T> TaskDataTrait for T where T : Any + Send{}

pub trait TaskFuncTrait : Fn(&TaskInfo, &Box<dyn TaskDataTrait>) -> TaskOutput + Send + Sync {}

impl<T> TaskFuncTrait for T where T : Fn(&TaskInfo, &Box<dyn TaskDataTrait>) -> TaskOutput + Send + Sync {}

pub trait TaskFuncTraitWithoutArgs : Fn(&TaskInfo) -> TaskOutput + Send + Sync {}

impl<T> TaskFuncTraitWithoutArgs for T where T : Fn(&TaskInfo) -> TaskOutput + Send + Sync {}

pub struct TaskMutexedProperties<'a> {
    func_ : Box<dyn TaskFuncTrait + 'a>,
    data_ : Box<dyn TaskDataTrait>,
}

pub struct TaskArcedProperties<'a>{
    invocations_ : AtomicUsize,
    executed : AtomicBool,
    mutexed_ : Mutex<TaskMutexedProperties<'a>>
}


#[derive(Clone)]
pub struct TaskOrderStruct {
    pub id_ : usize,
    priority_ : Arc<isize>,
}

impl std::fmt::Debug for TaskOrderStruct{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("task order struct")
         .field("id_", &self.id_)
         .field("priority_", &self.priority_).finish()
    }
}
#[derive(Clone)]
pub struct Task<'a> {
    id_ : usize,
    pub active : bool,
    data : Arc<TaskArcedProperties<'a>>,
    priority_ : Arc<isize>,
    type_ : TaskEnum,
    timestamp_ : time::Instant,
    interval_ : Option<f32>,
}

impl std::fmt::Debug for Task<'_>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = &self.data;
        f.debug_struct("task")
         .field("id_", &self.id_)
         .field("active", &self.active)
         .field("priority_", &self.priority_)
         .field("type_", &self.type_)
         .field("invocations", &data.invocations_)
         .field("timestamp", &{time::Instant::now() - self.timestamp_}.as_secs_f64())
         .finish()
    }
} 

impl cmp::PartialEq for TaskOrderStruct{
    fn eq (&self, other : &TaskOrderStruct) -> bool {
        self.id_ == other.id_ && -*self.priority_ == -*other.priority_
    }
}

impl cmp::Eq for TaskOrderStruct {}

impl cmp::PartialOrd for TaskOrderStruct {
    // Required method
    fn partial_cmp(&self, other: &TaskOrderStruct) -> Option<cmp::Ordering> {
        (-*self.priority_, self.id_).partial_cmp(&(-*other.priority_, other.id_))
    }
}

impl cmp::Ord for TaskOrderStruct {
    fn cmp(&self, other: &TaskOrderStruct) -> cmp::Ordering{
        (-*self.priority_, self.id_).cmp(&(-*other.priority_, other.id_))
    }
}

impl<'a> Task<'a> {
    pub fn new( func :Option<Box<dyn TaskFuncTrait + 'a>>, task_type : TaskEnum) -> Task {
        static ID_COUNT : AtomicUsize = AtomicUsize::new(1);
        Task{
            id_ : ID_COUNT.fetch_add(1, Ordering::Relaxed),
            active : true,
            timestamp_ : time::Instant::now(),
            interval_ : None,
            data : Arc::new(TaskArcedProperties{
                invocations_ : AtomicUsize::new(0),
                executed : AtomicBool::new(false),
                mutexed_ : Mutex::new(TaskMutexedProperties{
                    func_ : func.unwrap_or(Box::new(|_, _| {TaskOutput::Keep})),
                    data_ : Box::new(0)
                }),
            }),
            priority_ : Arc::new(0),
            type_ : task_type,
        }
    }
    pub fn set_func(&self, func : Box<dyn TaskFuncTrait + 'a>, data : Box<dyn TaskDataTrait>){
        let mut mutexed_data = self.data.mutexed_.lock().unwrap();
        mutexed_data.func_ = func;
        mutexed_data.data_ = data;
    }
    pub fn set_func_data(&self, data : Box<dyn TaskDataTrait>){
        let mut mutexed_data = self.data.mutexed_.lock().unwrap();
        mutexed_data.data_ = data;
    }   
    pub fn set_interval(&mut self, interval : Option<f32>){
        self.interval_ = interval;
    }
    pub fn id(&self) -> usize {
        self.id_
    }
    pub fn get_type(&self) -> TaskEnum {
        self.type_
    }

    pub fn interval(&self) -> Option<f32> {
        self.interval_
    }
    pub fn get_order_struct(&self) -> TaskOrderStruct{
        TaskOrderStruct{
            id_ : self.id_,
            priority_ : Arc::clone(&self.priority_)
        }
    }

    pub fn invocations(&self) -> usize{
        self.data.invocations_.load(Ordering::Relaxed)
    }

    pub fn update(&mut self, updated_time : &time::Instant){
        if self.data.executed.swap(false, Ordering::Relaxed){
            self.timestamp_ = *updated_time;
        }
    }

    pub fn execute(&self, updated_time : &time::Instant, thread_id : &usize) -> TaskOutput{
        if self.active {
            //println!("{:?}", &self);
            //let (delta, invoc, interval) = self.update_elapsed(updated_time);
            let data = &self.data;
            let interval = self.interval_;
            let cur_time = self.timestamp_;
            let (delta, invoc) = ({*updated_time - cur_time}.as_secs_f64(), data.invocations_.load(Ordering::Relaxed));
            match interval {
                    Some(i) => if (delta - i as f64) > 1e-8 {
                        data.invocations_.fetch_add(1, Ordering::Relaxed);
                        data.executed.store(true, Ordering::Relaxed);
                        let mutexed_data = data.mutexed_.lock().unwrap();
                        (mutexed_data.func_)(&TaskInfo {id_:self.id_, invocations_ : invoc+1, delta_ : delta, type_:self.type_, thread_id_:*thread_id }, &mutexed_data.data_)
                    }
                    else{
                        TaskOutput::Keep
                    }
                    None => {
                        data.invocations_.fetch_add(1, Ordering::Relaxed);
                        data.executed.store(true, Ordering::Relaxed);
                        let mutexed_data = data.mutexed_.lock().unwrap();
                        (mutexed_data.func_)(&TaskInfo {id_:self.id_, invocations_ : invoc+1, delta_ : delta, type_:self.type_, thread_id_:*thread_id }, &mutexed_data.data_)
                    }
            }
        }
        else {
            TaskOutput::Keep
        }
    }
}