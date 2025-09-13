use std::sync::{Arc, Mutex, MutexGuard};
use compact_str::CompactString;

use super::camera::*;
use super::light::*;

use super::object_trait::*;
use super::polygonstorage::{DynamicPolygonStorage, StaticPolygonStorage};
use super::polygonstoragetrait::*;

#[derive(Debug, Clone)]
pub struct Mesh {
    data_ : CommonObjectData,
    pub polygon_storage_ : (CompactString, Arc<Mutex<(Box<dyn PolygonStorageTrait>, bool)>>),
}

impl Mesh {
    pub fn new_static(positions : Vec<f32>, normals : Vec<f32>, uvmaps : Vec<UVMapData>, indices : Vec<u32>, vgroups : Vec<VertexGroup>, polygons_name : &str) -> Mesh{
        Mesh{
            data_ : CommonObjectData::new(ObjectType::Mesh),
            polygon_storage_ : (polygons_name.into(), Arc::new(Mutex::new((Box::new(StaticPolygonStorage::new(DynamicPolygonStorage::new(positions, normals, uvmaps, indices, vgroups))), true))))
        }
    }

    pub fn vertexgroup_names(&self) -> Vec<CompactString>{
        vec![]
    }

    pub fn get_polygonstorage_unlocked(&self) -> MutexGuard<'_, (Box<dyn PolygonStorageTrait>, bool)> {
        self.polygon_storage_.1.lock().unwrap()
    }
}

impl ObjectTrait for Mesh {
    fn get_camera(&self) -> Option<Camera> {None}
    fn get_light(&self) -> Option<Light> {None}
    fn get_mesh(&self) -> Option<Mesh> {Some(self.clone())}
    fn get_mesh_mut(&mut self) -> Option<&mut Mesh> {Some(self)}
    fn get_data(&self) -> &CommonObjectData {
        &self.data_
    }
    fn get_data_mut(&mut self) -> &mut CommonObjectData {
        &mut self.data_
    }
    fn update(&mut self){
        let mut polygons = self.polygon_storage_.1.lock().unwrap();
        polygons.1 = false;
    }
}