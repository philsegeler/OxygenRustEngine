use std::sync::{Arc, Mutex, LazyLock};

use super::object_trait::*;
use super::world::*;
use super::scene::*;
use super::material::*;
use super::viewport::*;
//use super::polygonstoragetrait::*;
use super::basecontainer::*;
use super::elementcontainer::*;

pub type InterpreterElementWrapper<T> = BaseContainer<Arc<SingleElement<T>>>;
type GlobalVar<T> = LazyLock<Arc<Mutex<T>>>;

#[derive(Debug, Default)]
pub struct GlobalScenegraphChanged {
    pub world_     : Option<World>,
    pub scenes_    : ElementSnapshot<Scene>,
    pub objects_   : ElementSnapshot<Box<dyn ObjectTrait>>,
    //pub polygons_  : ElementSnapshot<Box<dyn PolygonStorageTrait>>,
    pub materials_ : ElementSnapshot<Material>,
    pub viewports_ : ElementSnapshot<ViewPort>,
}
