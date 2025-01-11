use nohash_hasher::*;

use sorted_vec::*;
use std::ops::Index;
use std::time;
use std::sync::{Arc, atomic::{Ordering, AtomicU32}};

use super::task::*;
use super::types::basecontainer::BaseContainer;

#[derive(Debug, Default, Clone)]
pub struct TaskContainer<'a> {
    tasks_list_ : BaseContainer<Task<'a>>,
    sorted_tasks_ : SortedSet<TaskOrderStruct>,
    pub happened_tasks_counter_ : Arc<IntMap<usize, AtomicU32>>,
}

impl<'a> TaskContainer<'a> {
    pub fn ids(&self) -> Vec<usize> {
        self.tasks_list_.keys().cloned().collect()
    }
    pub fn len(&self) -> usize {
        self.tasks_list_.len()
    }
    pub fn contains(&self, task_id : &usize) -> bool{
        self.tasks_list_.contains(task_id)
    }
    pub fn contains_name(&self, task_name : &str) -> bool{
        self.tasks_list_.contains_name(task_name)
    }

    pub fn get_name(&self, task_id : &usize) -> Option<&str> {
        self.tasks_list_.get_name(task_id)
    }

    pub fn get_id(&self, task_name : &str) -> Option<usize> {
        self.tasks_list_.get_id(task_name)
    }

    pub fn insert_no_overwrite(&mut self, name : &str, func : Box<dyn TaskFuncTrait + 'a>, task_type : TaskEnum) -> usize{
        if !self.tasks_list_.contains_name(name) {
            let new_task = Task::new(Some(func), task_type);
            let output = new_task.id();
            self.tasks_list_.insert(new_task.id(), new_task.clone(), name);
            self.sorted_tasks_.push(new_task.get_order_struct());
            Arc::get_mut(&mut self.happened_tasks_counter_).unwrap().insert(output.clone(), AtomicU32::new(0));
            return output;
        }
        else{
            return 0;
        }
    }

    pub fn insert(&mut self, name : &str, func : Box<dyn TaskFuncTrait + 'a>, task_type : TaskEnum) -> usize {
        if self.tasks_list_.contains_name(name) {
            let task_id = self.tasks_list_.get_id(name).unwrap();
            let new_task = self.tasks_list_[&task_id].clone();
            new_task.set_func(func, Box::new(0));
            self.tasks_list_.insert(task_id, new_task.clone(), name);
            self.sorted_tasks_.push(new_task.get_order_struct());
            Arc::get_mut(&mut self.happened_tasks_counter_).unwrap().insert(task_id.clone(), AtomicU32::new(0));
            return task_id;
        }
        else{
            let new_task = Task::new(Some(func), task_type);
            let output = new_task.id();
            self.tasks_list_.insert(new_task.id(), new_task.clone(), name);
            self.sorted_tasks_.push(new_task.get_order_struct());
            Arc::get_mut(&mut self.happened_tasks_counter_).unwrap().insert(output.clone(), AtomicU32::new(0));
            return output;
        }
    }

    pub fn set_func(&self, task_id : &usize, func : Box<dyn TaskFuncTrait + 'a>, data : Box<dyn TaskDataTrait>) -> Option<bool>{
        let tasks_list = &self.tasks_list_;
        tasks_list.get(task_id)?.set_func(func, data);
        Some(true)
    }
    pub fn set_func_data(&self, task_id : &usize, data : Box<dyn TaskDataTrait>) -> Option<bool>{
        let tasks_list = &self.tasks_list_;
        tasks_list.get(task_id)?.set_func_data(data);
        Some(true)
    }
    pub fn set_interval(&mut self, task_id : &usize, interval : Option<f32>) -> Option<bool>{
        let tasks_list = &mut self.tasks_list_;
        tasks_list.get_mut(task_id)?.set_interval(interval);
        Some(true)
    }

    pub fn remove(&mut self, task_id : usize){
        let task = self.tasks_list_[&task_id].clone();
        self.tasks_list_.remove(&task_id);
        self.sorted_tasks_.remove_item(&task.get_order_struct());
    }
    pub fn clear(&mut self){
        self.tasks_list_.clear();
        self.sorted_tasks_.clear();
    }
    pub fn sorted(&self) -> &SortedSet<TaskOrderStruct>{
        &self.sorted_tasks_
    }

    pub fn update_task(&mut self, task_id : &usize, updated_time : &time::Instant) -> Option<bool>{
        let tasks_list = &mut self.tasks_list_;
        tasks_list.get_mut(task_id)?.update(updated_time);
        Some(true)            
    }

    pub fn consume(&self) -> TaskContainer<'a>{
        self.clone()
    }

    pub fn run_tasks(&self, thread_id : &usize, updated_time : &time::Instant) -> Vec<usize>{
        for key in self.happened_tasks_counter_.keys(){
            self.happened_tasks_counter_[key].store(0, Ordering::Relaxed);
        }

        let mut to_be_removed = vec![];
        //for task in self.tasks_.sorted().iter()
        self.sorted().iter().for_each(|task| {
            let task = &self[&task.id_];
            let task_output = task.execute(&updated_time, thread_id);
            if task_output == TaskOutput::Drop || task.get_type() == TaskEnum::Once {
                to_be_removed.push(task.id());
            }
            else{
                self.happened_tasks_counter_[&task.id()].fetch_add(1, Ordering::Relaxed);
            }
        });
        to_be_removed
    }
}

impl<'a> Index<&usize> for TaskContainer<'a> {
    type Output = Task<'a>;
    fn index(&self, task_id : &usize) -> &Self::Output {
        &self.tasks_list_[task_id]
    }
}

impl<'a> Index<&str> for TaskContainer<'a> {
    type Output = Task<'a>;
    fn index(&self, task_name : &str) -> &Self::Output {
        &self.tasks_list_[task_name]
    }
}