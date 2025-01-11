use std::sync::{Arc, Weak, Mutex, LazyLock};
use super::object_trait::*;
use super::world::*;
use super::scene::*;
use super::material::*;
use super::viewport::*;
use super::polygonstoragetrait::*;
use super::basecontainer::*;

pub type ElementWrapper<T> = BaseContainer<Weak<Mutex<T>>>;
pub type GlobalElementWrapper<T> = LazyLock<Arc<Mutex<ElementWrapper<T>>>>;

pub static OE_WORLD_   : LazyLock<Arc<Mutex<Option<World>>>> = LazyLock::new(||{Default::default()});

pub static OE_OBJECTS_ : GlobalElementWrapper<Box<dyn ObjectTrait>> = LazyLock::new(||{Default::default()});
pub static OE_POLYGONS_ : GlobalElementWrapper<Box<dyn PolygonStorageTrait>> = LazyLock::new(||{Default::default()});
pub static OE_SCENES_  : GlobalElementWrapper<Scene> = LazyLock::new(||{Default::default()});
pub static OE_MATERIALS_  : GlobalElementWrapper<Material> = LazyLock::new(||{Default::default()});
pub static OE_VIEWPORTS_  : GlobalElementWrapper<ViewPort> = LazyLock::new(||{Default::default()});