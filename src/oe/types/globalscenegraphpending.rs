use std::sync::Arc;

use compact_str::CompactString;
use multi_containers::HashMultiMap;

use super::object_trait::*;
use super::world::*;
use super::scene::*;
use super::material::*;
use super::viewport::*;
//use super::polygonstoragetrait::*;
use super::basecontainer::*;
use super::elementcontainer::*;


type PendingElements<T> = BaseContainer<Arc<SingleElement<T>>>;

// VERTEX GROUP MESH KEY
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct VertexGroupMeshKey{
    pub vgroup : CompactString,
    pub mesh : CompactString,
}

impl From<(CompactString, CompactString)> for VertexGroupMeshKey{
    fn from(value: (CompactString, CompactString)) -> Self {
        VertexGroupMeshKey { vgroup: (value.0), mesh: (value.1) }
    }
}


// GLOBAL SCENE GRAPH PENDING
#[derive(Default)]
pub struct GlobalScenegraphPending{
    pub world_     : Option<World>,
    pub scenes_    : PendingElements<Scene>,
    pub objects_   : PendingElements<Box<dyn ObjectTrait>>,
    //polygons_  : EPendingElements<Box<dyn PolygonStorageTrait>>,
    pub materials_ : PendingElements<Material>,
    pub viewports_ : PendingElements<ViewPort>,

    pub object2viewport      : HashMultiMap<CompactString, CompactString>,
    pub object2scene         : HashMultiMap<CompactString, CompactString>,
    pub object2object        : HashMultiMap<CompactString, CompactString>,
    pub material2scene       : HashMultiMap<CompactString, CompactString>,
    pub material2vertexgroup : HashMultiMap<CompactString, VertexGroupMeshKey>,
}

impl GlobalScenegraphPending{
    pub fn new_object(&mut self, id : usize, element : Arc<SingleElement<Box<dyn ObjectTrait>>>, name : CompactString, scene_name : &str){
        self.object2scene.insert(name.clone(), scene_name.into());
        self.objects_.insert_str(id, element, name);
    }
    pub fn new_material(&mut self, id : usize, element : Arc<SingleElement<Material>>, name : CompactString, scene_name : &str){
        self.material2scene.insert(name.clone(), scene_name.into());
        self.materials_.insert_str(id, element, name);
    }
    pub fn new_scene(&mut self, id : usize, element : Arc<SingleElement<Scene>>, name : CompactString){
        self.scenes_.insert_str(id, element, name);
    }
    pub fn new_viewport(&mut self, id : usize, element : Arc<SingleElement<ViewPort>>, name : CompactString){
        self.viewports_.insert_str(id, element, name);
    }
}

use std::fmt;
impl std::fmt::Debug for GlobalScenegraphPending{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output_string = String::new(); 
        
        output_string.push_str(&format!("___world_ {:?}\n", &self.world_));
        output_string.push_str(&format!("___scenes_ {:?}\n", &self.scenes_));
        output_string.push_str(&format!("___objects_ {:?}\n", &self.objects_));
        output_string.push_str(&format!("___materials_ {:?}\n", &self.materials_));
        output_string.push_str(&format!("___viewports_ {:?}\n", &self.viewports_));
        output_string.push_str(&format!("___object2viewport {:?}\n", &self.object2viewport));
        output_string.push_str(&format!("___object2scene {:?}\n", &self.object2scene));
        output_string.push_str(&format!("___object2object {:?}\n", &self.object2object));
        output_string.push_str(&format!("___material2scene {:?}\n", &self.material2scene));
        output_string.push_str(&format!("___material2vertexgroup {:?}\n", &self.material2vertexgroup));
         write!(f, "Global Pending Scenegraph\n{}", output_string)
    }
} 