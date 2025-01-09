use std::sync::{Arc, Weak, Mutex, LazyLock};
use super::object_trait::*;
use super::world::*;
use super::scene::*;
use super::material::*;
use super::viewport::*;
use super::polygonstoragetrait::*;
use super::basecontainer::*;

pub type ElementWrapper<T> = Arc<Mutex<BaseContainer<Weak<Mutex<T>>>>>;

pub static OE_WORLD_   : LazyLock<Arc<Mutex<Option<World>>>> = LazyLock::new(||{Default::default()});

pub static OE_OBJECTS_ : LazyLock<ElementWrapper<Box<dyn ObjectTrait>>> = LazyLock::new(||{Default::default()});
pub static OE_POLYGONS_ : LazyLock<ElementWrapper<Box<dyn PolygonStorageTrait>>> = LazyLock::new(||{Default::default()});
pub static OE_SCENES_  : LazyLock<ElementWrapper<Scene>> = LazyLock::new(||{Default::default()});
pub static OE_MATERIALS_  : LazyLock<ElementWrapper<Material>> = LazyLock::new(||{Default::default()});
pub static OE_VIEWPORTS_  : LazyLock<ElementWrapper<ViewPort>> = LazyLock::new(||{Default::default()});