use std::sync::{Arc, Mutex, LazyLock};

use compact_str::CompactString;
use multi_containers::HashMultiMap;

use super::object_trait::*;
use super::world::*;
use super::scene::*;
use super::material::*;
use super::viewport::*;
//use super::polygonstoragetrait::*;
use super::basecontainer::*;
use super::super::carbon::interpreter::Interpreter;
use super::elementcontainer::*;
use super::globalscenegraphchanged::*;
use super::globalscenegraphpending::*;

pub type InterpreterElementWrapper<T> = BaseContainer<Arc<SingleElement<T>>>;
type GlobalVar<T> = LazyLock<Arc<Mutex<T>>>;


#[derive(Debug, Default)]
pub struct GlobalScenegraph{
    world_     : Option<World>,
    scenes_    : ElementContainer<Scene>,
    objects_   : ElementContainer<Box<dyn ObjectTrait>>,
    //polygons_  : ElementContainer<Box<dyn PolygonStorageTrait>>,
    materials_ : ElementContainer<Material>,
    viewports_ : ElementContainer<ViewPort>,

    object2viewport      : HashMultiMap<CompactString, CompactString>,
    object2scene         : HashMultiMap<CompactString, CompactString>,
    object2object        : HashMultiMap<CompactString, CompactString>,
    material2scene       : HashMultiMap<CompactString, CompactString>,
    material2vertexgroup : HashMultiMap<CompactString, VertexGroupMeshKey>,

    pending_elements      : GlobalScenegraphPending,
    pending_interpreters_ : Vec<(Interpreter, CompactString)>,
}

impl GlobalScenegraph{
    pub fn update(&mut self, changed : bool) -> (GlobalScenegraphChanged, Vec<CompactString>) {
        // delete unnecessary
        /*self.scenes_.cleanup();
        self.objects_.cleanup();
        //self.polygons_.cleanup();
        self.materials_.cleanup();
        self.viewports_.cleanup();*/

        // THIS is the only place where deleting takes place
        // output
        let output = GlobalScenegraphChanged{
            world_ : self.world_.clone(),
            scenes_ : self.scenes_.get_changed_and_reset(changed),
            objects_ : self.objects_.get_changed_and_reset(changed),
            //polygons_ : self.polygons_.get_changed_and_reset(changed),
            materials_ : self.materials_.get_changed_and_reset(changed),
            viewports_ : self.viewports_.get_changed_and_reset(changed),
        };

        // NEW FRAME STARTS HERE

        // update interpreter
        let mut events = vec![];
        /*for (inter, event) in self.pending_interpreters_.drain(..){
            if inter.world.is_some(){
                self.world_ = inter.world;
            }
            for element in &inter.scenes_{
                self.scenes_.insert_str(element.0, element.2, element.1);
            }
            /*for element in &inter.polygons_{
                self.polygons_.insert_str(element.0, element.2, element.1);
            }*/
            for element in &inter.objects_{
                self.objects_.insert_str(element.0, element.2, element.1);
            }
            for element in &inter.materials_{
                self.materials_.insert_str(element.0, element.2, element.1);
            }
            for element in &inter.viewports_{
                self.viewports_.insert_str(element.0, element.2, element.1);
            }
            events.push(event);
        }*/

        // delete unnecessary
        /*self.scenes_.cleanup();
        self.objects_.cleanup();
        //self.polygons_.cleanup();
        self.materials_.cleanup();
        self.viewports_.cleanup();
        */
        (output, events)
    }

    fn new_object(&mut self, name : &str, scene_name : &str, obj: Arc<SingleElement<Box<dyn ObjectTrait>>>) {

    }

    pub fn add_interpreted(&mut self, new_data : Interpreter, event : CompactString){
        self.pending_interpreters_.push((new_data, event));

    }
}



/*pub static OE_WORLD_   : LazyLock<Arc<Mutex<Option<World>>>> = LazyLock::new(||{Default::default()});

pub static OE_OBJECTS_ : GlobalElementWrapper<Box<dyn ObjectTrait>> = LazyLock::new(||{Default::default()});
pub static OE_POLYGONS_ : GlobalElementWrapper<Box<dyn PolygonStorageTrait>> = LazyLock::new(||{Default::default()});
pub static OE_SCENES_  : GlobalElementWrapper<Scene> = LazyLock::new(||{Default::default()});
pub static OE_MATERIALS_  : GlobalElementWrapper<Material> = LazyLock::new(||{Default::default()});
pub static OE_VIEWPORTS_  : GlobalElementWrapper<ViewPort> = LazyLock::new(||{Default::default()});
pub static OE_PENDING_ELEMENTS_ : GlobalVar<Vec<Interpreter>> = LazyLock::new(||{Default::default()});*/