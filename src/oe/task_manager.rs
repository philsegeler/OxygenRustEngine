use std::sync::Arc;
use std::time;


use super::task_container::*;
use super::task::*;
use crate::oe;

fn task_default_fn(info: &TaskInfo, _:&Box<dyn TaskDataTrait>) -> TaskOutput{
    let task_name = oe::get_task_name(info.thread_id(), info.id());
    println!("{:?}", task_name);
    TaskOutput::Keep
}

#[derive(Default, Debug)]
pub struct TaskManager<'a>{
    tasks_: Arc<TaskContainer<'a>>,
}

impl<'a> TaskManager<'a>{
    pub fn create_task(&mut self, task_name : &str, type_in : TaskEnum) -> usize {
        let id =Arc::get_mut(&mut self.tasks_).unwrap().insert(&(task_name), Box::new(&task_default_fn), type_in);
        id
    }

    pub fn set_task_func(&self, task_id : &usize, func : impl TaskFuncTrait + 'a, data : Box<dyn TaskDataTrait>) -> Option<bool>{
        self.tasks_.set_func(task_id, Box::new(func), data)
    }
    pub fn set_task_data(&self, task_id : &usize, data : Box<dyn TaskDataTrait>) -> Option<bool>{
        self.tasks_.set_func_data(task_id, data)
    }
    pub fn set_task_interval(&mut self, task_id : &usize, interval : Option<f32>) -> Option<bool>{
        Arc::get_mut(&mut self.tasks_).unwrap().set_interval(task_id, interval)
    }

    pub fn get_task_name(&self, task_id : &usize) -> Option<&str> {
        self.tasks_.get_name(task_id)
    }
    pub fn get_task_id(&self, task_name : &str) -> Option<&usize> {
        self.tasks_.get_id(task_name)
    }

    pub fn update_task_timestamps(&mut self, updated_time : &time::Instant){
        for task_id in self.tasks_.ids(){
            Arc::get_mut(&mut self.tasks_).unwrap().update_task(&task_id, updated_time);
        }

    }
    pub fn consume_tasks(&self) -> TaskContainer<'a>{
        self.tasks_.consume()
    }
    pub fn consume_tasks_as_is(&self) -> Arc<TaskContainer<'a>>{
        Arc::clone(&self.tasks_)
    }

    pub fn remove_tasks(&mut self, to_be_removed : Vec<usize>) {
        let tasks = Arc::get_mut(&mut self.tasks_).unwrap();
        for task_id in to_be_removed{
            tasks.remove(task_id);
        }
    }
}