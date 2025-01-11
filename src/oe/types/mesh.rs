use std::sync::{Arc, Mutex};

use super::object_trait::*;
use super::polygonstorage::{DynamicPolygonStorage, StaticPolygonStorage};
use super::polygonstoragetrait::*;

#[derive(Debug)]
pub struct Mesh {
    data_ : CommonObjectData,
    polygon_storage_ : (usize, Arc<Mutex<Box<dyn PolygonStorageTrait>>>),
}

impl Mesh {
    pub fn new_static(positions : Vec<f32>, normals : Vec<f32>, uvmaps : Vec<UVMapData>, indices : Vec<u32>, vgroups : Vec<VertexGroup>) -> Mesh{
        Mesh{
            data_ : CommonObjectData::new(ObjectType::Mesh),
            polygon_storage_ : (0, Arc::new(Mutex::new(Box::new(StaticPolygonStorage::new(DynamicPolygonStorage::new(positions, normals, uvmaps, indices, vgroups))))))
        }
    }
}

impl ObjectTrait for Mesh {
    fn get_data(&self) -> &CommonObjectData {
        &self.data_
    }
    fn get_data_mut(&mut self) -> &mut CommonObjectData {
        &mut self.data_
    }
}