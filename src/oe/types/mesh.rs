use std::sync::{Arc, Mutex};
use compact_str::CompactString;

use super::camera::*;
use super::light::*;

use super::object_trait::*;
use super::polygonstorage::{DynamicPolygonStorage, StaticPolygonStorage};
use super::polygonstoragetrait::*;

#[derive(Debug, Clone)]
pub struct Mesh {
    data_ : CommonObjectData,
    polygon_storage_ : (CompactString, Arc<Mutex<(Box<dyn PolygonStorageTrait>, bool)>>),
}

impl Mesh {
    pub fn new_static(positions : Vec<f32>, normals : Vec<f32>, uvmaps : Vec<UVMapData>, indices : Vec<u32>, vgroups : Vec<VertexGroup>, polygons_name : &str) -> Mesh{
        Mesh{
            data_ : CommonObjectData::new(ObjectType::Mesh),
            polygon_storage_ : (polygons_name.into(), Arc::new(Mutex::new((Box::new(StaticPolygonStorage::new(DynamicPolygonStorage::new(positions, normals, uvmaps, indices, vgroups))), true))))
        }
    }
}

impl ObjectTrait for Mesh {
    fn get_camera(&self) -> Option<Camera> {None}
    fn get_light(&self) -> Option<Light> {None}
    fn get_mesh(&self) -> Option<Mesh> {Some(self.clone())}
    fn get_data(&self) -> &CommonObjectData {
        &self.data_
    }
    fn get_data_mut(&mut self) -> &mut CommonObjectData {
        &mut self.data_
    }
}