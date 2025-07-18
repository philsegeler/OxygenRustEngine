use std::sync::{Arc, Mutex, LazyLock};

use super::object_trait::*;
use super::world::*;
use super::scene::*;
use super::material::*;
use super::viewport::*;
use super::polygonstoragetrait::*;
use super::basecontainer::*;
use super::super::carbon::interpreter::Interpreter;
use super::elementcontainer::*;

pub type ElementWrapper<T> = ElementContainer<T>;
pub type InterpreterElementWrapper<T> = BaseContainer<Arc<Mutex<(T, bool)>>>;
type GlobalVar<T> = LazyLock<Arc<Mutex<T>>>;

#[derive(Debug, Default)]
pub struct GlobalScenegraph{
    world_     : Option<World>,
    scenes_    : ElementWrapper<Scene>,
    objects_   : ElementWrapper<Box<dyn ObjectTrait>>,
    polygons_  : ElementWrapper<Box<dyn PolygonStorageTrait>>,
    materials_ : ElementWrapper<Material>,
    viewports_ : ElementWrapper<ViewPort>,

    pending_elements_ : Vec<Interpreter>,
}

#[derive(Debug, Default)]
pub struct GlobalScenegraphSimple {
    world_     : Option<World>,
    scenes_    : BaseContainer<Scene>,
    objects_   : BaseContainer<Box<dyn ObjectTrait>>,
    polygons_  : BaseContainer<Box<dyn PolygonStorageTrait>>,
    materials_ : BaseContainer<Material>,
    viewports_ : BaseContainer<ViewPort>,

}


impl GlobalScenegraph{
    pub fn update(&mut self, changed : bool) -> GlobalScenegraphSimple {
        // delete unnecessary
        self.scenes_.cleanup();
        self.objects_.cleanup();
        self.polygons_.cleanup();
        self.materials_.cleanup();
        self.viewports_.cleanup();

        // output
        let output = GlobalScenegraphSimple{
            world_ : self.world_.clone(),
            scenes_ : self.scenes_.get_changed_and_reset(changed),
            objects_ : self.objects_.get_changed_and_reset(changed),
            polygons_ : self.polygons_.get_changed_and_reset(changed),
            materials_ : self.materials_.get_changed_and_reset(changed),
            viewports_ : self.viewports_.get_changed_and_reset(changed),
        };

        // update interpreter
        for inter in self.pending_elements_.drain(..){
            if inter.world.is_some(){
                self.world_ = inter.world;
            }
            for element in &inter.scenes_{
                self.scenes_.insert_str(element.0, Arc::downgrade(&element.2), element.1);
            }
            for element in &inter.polygons_{
                self.polygons_.insert_str(element.0, Arc::downgrade(&element.2), element.1);
            }
            for element in &inter.objects_{
                self.objects_.insert_str(element.0, Arc::downgrade(&element.2), element.1);
            }
            for element in &inter.materials_{
                self.materials_.insert_str(element.0, Arc::downgrade(&element.2), element.1);
            }
            for element in &inter.viewports_{
                self.viewports_.insert_str(element.0, Arc::downgrade(&element.2), element.1);
            }
        }

        // delete unnecessary
        self.scenes_.cleanup();
        self.objects_.cleanup();
        self.polygons_.cleanup();
        self.materials_.cleanup();
        self.viewports_.cleanup();

        output
    }

    pub fn add_interpreted(&mut self, new_data : Interpreter){
        self.pending_elements_.push(new_data);

    }
}



/*pub static OE_WORLD_   : LazyLock<Arc<Mutex<Option<World>>>> = LazyLock::new(||{Default::default()});

pub static OE_OBJECTS_ : GlobalElementWrapper<Box<dyn ObjectTrait>> = LazyLock::new(||{Default::default()});
pub static OE_POLYGONS_ : GlobalElementWrapper<Box<dyn PolygonStorageTrait>> = LazyLock::new(||{Default::default()});
pub static OE_SCENES_  : GlobalElementWrapper<Scene> = LazyLock::new(||{Default::default()});
pub static OE_MATERIALS_  : GlobalElementWrapper<Material> = LazyLock::new(||{Default::default()});
pub static OE_VIEWPORTS_  : GlobalElementWrapper<ViewPort> = LazyLock::new(||{Default::default()});
pub static OE_PENDING_ELEMENTS_ : GlobalVar<Vec<Interpreter>> = LazyLock::new(||{Default::default()});*/