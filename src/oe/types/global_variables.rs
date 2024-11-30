use std::sync::{Arc, Weak, Mutex, LazyLock};
use super::object_trait::*;
use super::world::*;
use super::scene::*;
use super::material::*;
use super::polygonstoragetrait::*;
use super::basecontainer::*;

type ElementWrapper<T> = LazyLock<Arc<Mutex<BaseContainer<Weak<Mutex<T>>>>>>;

pub static OE_WORLD_   : LazyLock<Arc<Mutex<Option<World>>>> = LazyLock::new(||{Default::default()});

pub static OE_OBJECTS_ : ElementWrapper<Box<dyn ObjectTrait>> = LazyLock::new(||{Default::default()});
pub static OE_POLYGONS_ : ElementWrapper<Box<dyn PolygonStorageTrait>> = LazyLock::new(||{Default::default()});
pub static OE_SCENES_  : ElementWrapper<Scene> = LazyLock::new(||{Default::default()});
pub static OE_MATERIALS_  : ElementWrapper<Material> = LazyLock::new(||{Default::default()});