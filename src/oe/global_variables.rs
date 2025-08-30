
use core::cell::RefCell;
use std::sync::{Arc, Mutex, atomic::AtomicBool, LazyLock};
use super::types::global_scenegraph::GlobalScenegraph;

//use no_deadlocks::Mutex;
use super::base_traits::*;
use super::task_manager::*;
use super::dummy_structs::*;
use super::event_handler::*;
use std::thread;
//use super::types::global_scenegraph::*;
//trait OE_RendererBaseTrait : Send{}
//struct oe_renderer_init_info_t {x : i32}

// STATIC VARIABLES
pub static OE_SCENEGRAPH_ : LazyLock<Arc<Mutex<GlobalScenegraph>>> = LazyLock::new(||{Arc::new(Mutex::new(Default::default()))});

pub const OE_USE_MULTIPLE_THREADS_ : bool = true;

pub static OE_THREAD_HANDLE_   : LazyLock<UltimateWrapper<thread::JoinHandle<()>>> = LazyLock::new(||{new_ultimate_wrapper(None)});
pub static OE_START_CONDITION_ : LazyLock<Arc<MutexCondition>> = LazyLock::new(||{Arc::new(MutexCondition::new(2))});
pub static OE_END_CONDITION_   : LazyLock<Arc<MutexCondition>> = LazyLock::new(||{Arc::new(MutexCondition::new(2))});

thread_local!{
    pub static OE_WINSYS_ : RefCell<Box<dyn WinsysBaseTrait>> = RefCell::new(Box::new(DummyWinsys{})); 
}
pub static OE_EVENT_HANDLER_ : LazyLock<UltimateWrapper<EventHandler>> = LazyLock::new(||{new_ultimate_wrapper(None)});
//pub static OE_TASK_MANAGER_ : LazyLock<UltimateWrapper<TaskManager>> = LazyLock::new(||{new_ultimate_wrapper(None)});
pub static OE_TASK_MANAGERS_ : LazyLock<TaskManagerList<TaskManager>> = LazyLock::new(||{new_task_manager_list()});
pub static OE_UNSYNC_THREADS_ : LazyLock<Arc<Mutex<Vec<std::thread::JoinHandle<()>>>>> = LazyLock::new(||{Default::default()});

//pub static OE_RENDERER_   : TraitWrapper<dyn OE_RendererBaseTrait> = Mutex::new(None);
//pub static OE_PHYSICS_    : TraitWrapper<dyn OE_PhysicsBaseTrait> = Mutex::new(None);
//pub static OE_NETWORKING_ : TraitWrapper<dyn OE_NetworkingBaseTrait> = Mutex::new(None);

pub static OE_DONE_ : AtomicBool = AtomicBool::new(false);  

pub static OE_WINSYS_INIT_INFO_   : LazyLock<Mutex<Option<WinsysInitInfo>>> = LazyLock::new(||{Mutex::new(None)});    
pub static OE_WINSYS_UPDATE_INFO_ : LazyLock<Mutex<Option<WinsysUpdateInfo>>> =LazyLock::new(||{Mutex::new(None)}); 
pub static OE_WINSYS_OUTPUT_INFO_ : LazyLock<Mutex<Option<WinsysOutput>>> = LazyLock::new(||{Mutex::new(None)});          

//pub static OE_RENDERER_INIT_INFO_ : Mutex<Option<oe_renderer_init_info_t>> = Mutex::new(None);
//pub static OE_RENDERER_UPDATE_INFO_: Mutex<Option<oe_renderer_update_info_t>> = Mutex::new(None);

//pub static OE_NETWORKING_INIT_INFO_ : Mutex<Option<oe_networking_init_info_t>> = Mutex::new(None);
//pub static OE_PHYSICS_INIT_INFO_ : Mutex<Option<oe_physics_init_info_t>> = Mutex::new(None);
     
//pub static OE_PHYSICS_UPDATE_INFO_ : Mutex<Option<oe_physics_update_info_t>> = Mutex::new(None);