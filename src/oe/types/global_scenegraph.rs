use std::sync::{Arc, Weak, Mutex, LazyLock};
use crate::oe::types::polygonstorage::StaticPolygonStorage;

use super::object_trait::*;
use super::world::*;
use super::scene::*;
use super::material::*;
use super::viewport::*;
use super::polygonstoragetrait::*;
use super::basecontainer::*;
use super::super::carbon::interpreter::Interpreter;


pub type ElementWrapper<T> = BaseContainer<Weak<Mutex<T>>>;
pub type GlobalElementWrapper<T> = LazyLock<Arc<Mutex<ElementWrapper<T>>>>;
type GlobalVar<T> = LazyLock<Arc<Mutex<T>>>;

#[derive(Default, Clone)]
pub struct GlobalScenegraph{
    world_     : Option<World>,
    scenes_    : ElementWrapper<Scene>,
    objects_   : ElementWrapper<Box<dyn ObjectTrait>>,
    polygons_  : ElementWrapper<Box<dyn PolygonStorageTrait>>,
    materials_ : ElementWrapper<Material>,
    viewports_ : ElementWrapper<ViewPort>,

    pending_elements_ : Vec<Interpreter>,
}

pub struct GlobalScenegraphSimple {
    world_     : Option<World>,
    scenes_    : BaseContainer<Scene>,
    objects_   : BaseContainer<Box<dyn ObjectTrait>>,
    polygons_  : BaseContainer<StaticPolygonStorage>,
    materials_ : BaseContainer<Material>,
    viewports_ : BaseContainer<ViewPort>,

}


impl GlobalScenegraph{
    pub fn update(&mut self) -> GlobalScenegraphSimple {
        // delete unnecessary

        // output
        let output = GlobalScenegraphSimple{
            world_ : self.world_.clone(),
            scenes_ : self.scenes_.get_real(),
            objects_ : self.objects_.get_real(),
            polygons_ : self.polygons_.get_real(),
            materials_ : self.materials_.get_real(),
            viewports_ : self.viewports_.get_real(),
        };
        // update interpreter
        output
    }
}



/*pub static OE_WORLD_   : LazyLock<Arc<Mutex<Option<World>>>> = LazyLock::new(||{Default::default()});

pub static OE_OBJECTS_ : GlobalElementWrapper<Box<dyn ObjectTrait>> = LazyLock::new(||{Default::default()});
pub static OE_POLYGONS_ : GlobalElementWrapper<Box<dyn PolygonStorageTrait>> = LazyLock::new(||{Default::default()});
pub static OE_SCENES_  : GlobalElementWrapper<Scene> = LazyLock::new(||{Default::default()});
pub static OE_MATERIALS_  : GlobalElementWrapper<Material> = LazyLock::new(||{Default::default()});
pub static OE_VIEWPORTS_  : GlobalElementWrapper<ViewPort> = LazyLock::new(||{Default::default()});
pub static OE_PENDING_ELEMENTS_ : GlobalVar<Vec<Interpreter>> = LazyLock::new(||{Default::default()});*/