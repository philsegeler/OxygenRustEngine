//use core::cell::RefCell;

use std::sync::Arc;
use std::sync::{Condvar, Mutex};
//use no_deadlocks::{Condvar, Mutex};
use std::sync::RwLock;

//use parking_lot::ReentrantMutex;
//use no_deadlocks::RwLock;
//use parking_lot::ReentrantMutex;

//pub type UltimateWrapper<T> = Arc<ReentrantMutex<RefCell<Option<T>>>>;
pub type UltimateWrapper<T> = Arc<RwLock<Option<T>>>;
//pub type TaskManagerWrapper<T> = Arc<ReentrantMutex<RefCell<T>>>;

pub struct MutexCondition{
    value : Mutex<i8>,
    max_value : i8,
    condition : Condvar,
}

impl MutexCondition{
    pub fn new(max_value : i8) -> MutexCondition{
        MutexCondition{
            value : Mutex::new(0),
            max_value,
            condition : Condvar::new()
        }
    }

    pub fn release(&self){
        let mut value = self.value.lock().unwrap();
        *value += 1;
        self.condition.notify_all();
    }

    pub fn update(&self){
        let mut value = self.value.lock().unwrap();
        *value += 1;
        
        if *value < self.max_value{
            //println!("entering");


                let _x = self.condition.wait(value).unwrap();

        }
        else {
            //println!("exit");
            *value = 0;
            self.condition.notify_all();
        }
    }
}


#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum WinsysBackend {
    #[default]
    Angle,
    Gles2,
    Gl3,
    Wgpu
}

#[derive(Default)]
pub struct WinsysInitInfo{
    pub requested_backend : WinsysBackend,
}

#[derive(Clone, Debug)]
pub struct WinsysUpdateInfo{
    pub title: String,
    pub res_x: u32,
    pub res_y: u32,
    
    pub res_changed : bool,

    pub use_fullscreen : bool,

    pub vsync : bool,
    pub mouse_locked : bool,
}

impl Default for WinsysUpdateInfo {
    fn default() -> Self {WinsysUpdateInfo{ title: "".to_string(), 
                            res_x:0, 
                            res_y:0, 
                            res_changed : false,
                            use_fullscreen:false,
                            vsync:true,
                            mouse_locked:false,  
                        }}
}

#[derive(Clone, Default, Debug)]
pub struct WinsysOutput{
    pub update_info : WinsysUpdateInfo,

    pub major : u16,
    pub minor : u16,
    pub dpi : u16,

    pub backend : WinsysBackend,
    pub mouse_moved : bool,
    pub done        : bool,
}

#[derive(Clone, Debug, Copy, Default)]
pub enum RendererShadingMode{
    #[default]
    Regular,
    Normals
}

#[derive(Clone, Debug, Copy, Default)]
pub struct RendererUpdateInfo{
    pub use_hdr : bool,
    pub use_wireframe : bool,
    pub use_light_indexed_rendering : bool,
    pub render_bounding_boxes : bool,
    pub render_bounding_spheres : bool,
    pub use_z_prepass : bool,
    pub restart_renderer : bool,
    pub shading_mode : RendererShadingMode,
}