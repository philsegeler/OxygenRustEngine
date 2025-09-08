use std::sync::{Arc, Mutex, LazyLock};
use std::collections::HashSet;
    use std::iter::Iterator;

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


#[derive(Default)]
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

    pending_events        : Vec<usize>,
    pending_elements      : GlobalScenegraphPending,
    pending_interpreters_ : Vec<(Interpreter, usize)>,
}

impl GlobalScenegraph{
    pub fn update(&mut self, changed : bool) -> (GlobalScenegraphChanged, Vec<usize>) {

        // output
        let output = GlobalScenegraphChanged{
            world_ : self.world_.clone(),
            scenes_ : self.scenes_.get_changed_and_reset(changed),
            objects_ : self.objects_.get_changed_and_reset(changed),
            //polygons_ : self.polygons_.get_changed_and_reset(changed),
            materials_ : self.materials_.get_changed_and_reset(changed),
            viewports_ : self.viewports_.get_changed_and_reset(changed),
        };

        // delete everything that is necessary
        for name in std::mem::take(self.scenes_.get_deleted()){
            self.remove_scene(&name);
        }
        for name in std::mem::take(self.materials_.get_deleted()){
            self.remove_material(&name);
        }
        for name in std::mem::take(self.objects_.get_deleted()){
            self.remove_object(&name);
        }
        for name in std::mem::take(self.viewports_.get_deleted()){
            self.remove_viewport(&name);
        }

        // NEW FRAME STARTS HERE
        
        // consume pending on the fly data
        let mut pending_elements = std::mem::take(&mut self.pending_elements);
        self.consume_pending_elements(&mut pending_elements);

        // consume interpreters
        let mut events = std::mem::take(&mut self.pending_events);
        //println!("234 {:?}\n", self);
        let pending_interpreters = std::mem::take(&mut self.pending_interpreters_);
        for (mut inter, event) in pending_interpreters{
            self.consume_pending_elements(inter.get_data());
            println!("{:?}", self);
            events.push(event);
        }

        (output, events)
    }
    fn consume_pending_elements(&mut self, data : &mut GlobalScenegraphPending){
        
        // update world -> nuke everything
        
        if data.world_.is_some(){
            for name in self.viewports_.names(){
                self.remove_viewport(&name);
            }
            
            for name in self.scenes_.names(){
                self.remove_scene(&name);
            }
            
            self.world_ = data.world_.clone();
        }
        

        // extend hashmaps for linked elements
        self.object2viewport.extend(std::mem::take(&mut data.object2viewport).mappings().into_iter().map(|(a, b)| (a.clone(), b.clone())));
        self.object2scene.extend(std::mem::take(&mut data.object2scene).mappings().into_iter().map(|(a, b)| (a.clone(), b.clone())));
        self.object2object.extend(std::mem::take(&mut data.object2object).mappings().into_iter().map(|(a, b)| (a.clone(), b.clone())));
        self.material2scene.extend(std::mem::take(&mut data.material2scene).mappings().into_iter().map(|(a, b)| (a.clone(), b.clone())));
        self.material2vertexgroup.extend(std::mem::take(&mut data.material2vertexgroup).mappings().into_iter().map(|(a, b)| (a.clone(), b.clone())));
        
        // update elements
        for (id, name, element) in &data.scenes_ {
            self.new_scene(id, name.clone(), element, data).expect(&(CompactString::new("Scene : \"") + &name + &"\" could not be added."));
        }
        for (id, name, obj) in &data.objects_ {
            self.new_object(id, name.clone(), obj, data).expect(&(CompactString::new("Object : \"") + &name + &"\" could not be added."));
        }
        for (id, name, obj) in &data.materials_ {
            self.new_material(id, name.clone(), obj, data).expect(&(CompactString::new("Material : \"") + &name + &"\" could not be added."));
        }
        for (id, name, obj) in &data.viewports_ {
            self.new_viewport(id, name.clone(), obj, data).expect(&(CompactString::new("Viewport : \"") + &name + &"\" could not be added."));
        }
        
        std::mem::take(data);
    }

    // HANDLE INDIVIDUAL OBJECTS
    fn new_object(&mut self, id : usize, name : CompactString, element: Arc<SingleElement<Box<dyn ObjectTrait>>>, data : &GlobalScenegraphPending) -> Result<u8, String> {
        
        let object_unlocked = element.lock().unwrap();
        
        if let Some(old_id) = self.objects_.get_id(&name){
            let old_object = self.objects_.remove_now(&old_id).unwrap();
            let old_object_unlocked = old_object.0.lock().unwrap();

            // handle unlinked objects
            for old_obj_name in old_object_unlocked.0.get_linked_objects().difference(&object_unlocked.0.get_linked_objects()){
                self.object2object.remove(old_obj_name, &name);
                if self.object2object.get(old_obj_name).is_none(){
                    self.remove_object(old_obj_name);
                }
            }
        }
        self.check_object_validity(object_unlocked.0.get_linked_objects().iter(), data, "Object")?;

        // finally add object
        drop(object_unlocked);
        if let Some(names) = self.object2scene.get(&name){
            if self.scenes_.contains_names(names.iter()){
                self.objects_.insert_str(id, element, name);
            }
            else {
                return Err("Object belongs in non-existent scene.".to_string());
            }
        }
        Ok(5)
    }
    fn remove_object(&mut self, name : &str){
        if let Some(old_id) = self.objects_.get_id(&name){
            self.objects_.remove_now(&old_id);

            for scenekey in self.object2scene.remove_key(name).unwrap_or_default(){
                let scene_id = self.scenes_.get_id(&scenekey).unwrap();
                let mut scene = self.scenes_[scene_id].lock().unwrap();
                scene.0.objects.remove(name);
            }
            for objectkey in self.object2object.remove_key(name).unwrap_or_default(){
                let object_id = self.objects_.get_id(&objectkey).unwrap();
                let object = self.objects_[object_id].lock().unwrap();
                //TODO UNLINK OBJECT
                //scene.0.materials.remove(name);

                // remove if parent is deleted
                if object.0.get_parent() == name {
                    drop(object);
                    self.remove_object(&objectkey);
                }
            }
        }
    }
    fn new_scene(&mut self, id : usize, name : CompactString, element: Arc<SingleElement<Scene>>, data : &GlobalScenegraphPending) -> Result<u8, String> {
        
        let scene_unlocked = element.lock().unwrap();
        if let Some(old_id) = self.scenes_.get_id(&name){
            //println!("{:?}", self);
            let old_scene = self.scenes_.remove_now(&old_id).unwrap();          
            let old_scene_unlocked = old_scene.0.lock().unwrap();

            // handle unlinked objects
            for old_obj_name in old_scene_unlocked.0.objects.difference(&scene_unlocked.0.objects){
                self.object2scene.remove(old_obj_name, &name);
                if self.object2scene.get(old_obj_name).is_none(){
                    self.remove_object(old_obj_name);
                }
            }

            // handle unlinked materials
            for old_obj_name in old_scene_unlocked.0.materials.difference(&scene_unlocked.0.materials){
                self.material2scene.remove(old_obj_name, &name);
                if self.material2scene.get(old_obj_name).is_none(){
                    self.remove_material(old_obj_name);
                }
            }
        }
        
        self.check_object_validity(scene_unlocked.0.objects.iter(), data, "Scene")?;
        self.check_material_validity(scene_unlocked.0.materials.iter(), data, "Scene")?;

        // finally add scene
        drop(scene_unlocked);
        self.scenes_.insert_str(id, element, name);

        Ok(5)
    }
    fn remove_scene(&mut self, name : &str){
        if let Some(old_id) = self.scenes_.get_id(&name){
            let scene = self.scenes_.remove_now(&old_id).unwrap();
            let scene_unlocked = scene.0.lock().unwrap();

            for obj_name in &scene_unlocked.0.objects{
                self.object2scene.remove(obj_name, name);
                if ! self.object2scene.contains_key(obj_name){
                    self.remove_object(&obj_name);
                }
            }
            for material_name in &scene_unlocked.0.materials{
                self.material2scene.remove(material_name, name);
                if ! self.material2scene.contains_key(material_name){
                    self.remove_material(&material_name);
                }
            }
        }
    }
    fn new_material(&mut self, id : usize, name : CompactString,  element: Arc<SingleElement<Material>>, _ : &GlobalScenegraphPending) -> Result<u8, String> {
        //TODO : IF VALID LINKS EXIST
        if let Some(names) = self.material2scene.get(&name){
            if self.scenes_.contains_names(names.iter()){
                self.materials_.insert_str(id, element, name);
            }
            else {
                return Err("Material belongs to non-existent scene.".to_string());
            }
        }
        Ok(5)
    }
    fn remove_material(&mut self, name : &str){
        if let Some(old_id) = self.materials_.get_id(&name){
            self.materials_.remove_now(&old_id);

            for vgroupmeshkey in self.material2vertexgroup.remove_key(name).unwrap_or_default(){
                let obj_id = self.objects_.get_id(&vgroupmeshkey.mesh);
                if obj_id.is_none() {continue;}
                let mut object = self.objects_[obj_id.unwrap()].lock().unwrap();
                let mesh = object.0.get_mesh_mut().unwrap();
                let mut polygons = mesh.get_polygonstorage_unlocked();

                for vgroup in polygons.0.get_vgroups_mut(){
                    if vgroup.material == Some(name.into()){
                        vgroup.material = None;
                    }
                }
            }
            for scenekey in self.material2scene.remove_key(name).unwrap_or_default(){
                let scene_id = self.scenes_.get_id(&scenekey).unwrap();
                let mut scene = self.scenes_[scene_id].lock().unwrap();
                scene.0.materials.remove(name);
            }
       }
    }
    fn new_viewport(&mut self, id : usize, name : CompactString, element: Arc<SingleElement<ViewPort>>, data : &GlobalScenegraphPending) -> Result<u8, String> {
        
        let viewport_unlocked = element.lock().unwrap();
        
        if let Some(old_id) = self.viewports_.get_id(&name){
            let old_viewport = self.viewports_.remove_now(&old_id).unwrap();
            let old_viewport_unlocked = old_viewport.0.lock().unwrap();

            // handle unlinked objects
            for old_obj_name in HashSet::<CompactString>::from_iter(old_viewport_unlocked.0.cameras_.iter().cloned()).difference(&HashSet::from_iter(viewport_unlocked.0.cameras_.iter().cloned())){
                self.object2viewport.remove(old_obj_name, &name);
                if self.object2viewport.get(old_obj_name).is_none(){
                    self.remove_object(old_obj_name);
                }
            }
        }

        self.check_object_validity(viewport_unlocked.0.cameras_.iter(), data, "Viewport")?;

        // finally add viewport
        drop(viewport_unlocked);
        self.viewports_.insert_str(id, element, name);

        Ok(5)
    }
    fn remove_viewport(&mut self, name : &str){
       if let Some(old_id) = self.viewports_.get_id(&name){
            self.viewports_.remove_now(&old_id);
       }
    }
    pub fn add_interpreted(&mut self, new_data : Interpreter, event : usize){
        self.pending_interpreters_.push((new_data, event));
    }

    /////////////////////////////////////////////////////////
    // internal specific functions
    fn check_object_validity<'a>(&self, objects_list : impl Iterator<Item=&'a CompactString>, data : &GlobalScenegraphPending, component_name : &str) -> Result<u8, String>{
        for obj in objects_list{
            if ! (self.objects_.contains_name(obj) || data.objects_.contains_name(obj)) {
                if component_name != "Object" {
                    return Err(String::from(component_name) + " does not contain object: \"" + obj + &"\". ");
                }
                else {
                    return Err(String::from("Object does not link to object: \"") + obj + &"\". ");
                }
            }
        }
        Ok(5)
    }
    fn check_material_validity<'a>(&self, materials_list : impl Iterator<Item=&'a CompactString>, data : &GlobalScenegraphPending, component_name : &str) -> Result<u8, String>{
        for obj in materials_list{
            if ! (self.materials_.contains_name(obj) || data.materials_.contains_name(obj)) {
                return Err(String::from(component_name) + " does not contain material: \"" + obj + &"\". ");
            }
        }
        Ok(5)
    }
}

fn test_global_scenegraph(){

}

use std::fmt;
impl std::fmt::Debug for GlobalScenegraph{
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
        output_string.push_str(&format!("___pending_interpreters {:?}\n", &self.pending_interpreters_));
        output_string.push_str(&format!("___pending_events {:?}\n", &self.pending_elements));
         write!(f, "Global Scenegraph\n{}", output_string)
    }
} 