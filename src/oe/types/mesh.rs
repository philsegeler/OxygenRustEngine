use std::sync::{Arc, Mutex};

use super::object_trait::*;
use super::polygonstorage::StaticPolygonStorage;
use super::polygonstoragetrait::*;

struct Mesh {
    data_ : CommonObjectData,
    polygon_storage_ : (usize, Arc<Mutex<Box<dyn PolygonStorageTrait>>>),
}

impl Mesh {
    fn new() -> Mesh{
        Mesh{
            data_ : CommonObjectData::new(),
            polygon_storage_ : (0, Arc::new(Mutex::new(Box::new(StaticPolygonStorage::default()))))
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