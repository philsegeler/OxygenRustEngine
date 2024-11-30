//use std::collections::HashSet;
use super::polygonstoragetrait::*;
use nohash_hasher::IntMap;
use std::ops::Index;

// polygon vertex key
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolygonVertexKey<'a>{
    data : &'a[u32]
}

impl<'a> PolygonVertexKey<'a>{
    fn new(data : &'a[u32]) -> PolygonVertexKey{
        PolygonVertexKey{
            data
        }
    }
}
impl<'a> Index<usize> for PolygonVertexKey<'a> {
    type Output = u32;
    fn index(&self, id : usize) -> &Self::Output {
        &self.data[id]
    }
}

impl<'a> std::hash::Hash for PolygonVertexKey<'a> {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        let mut output = [0;8];
        for i in 0..self.data.len(){
            let mut temp= (self.data[i] as u64).to_le_bytes();
            if i % 4 >= 2{
                temp = (self.data[i] as u64).to_be_bytes();
            }
            if i % 2 != 0{
                temp = ((self.data[i] as u64) << 32).to_le_bytes();
            } 
            for j in 0..8{
                output[j] |= temp[j];
            }
        }
        hasher.write_u64(u64::from_le_bytes(output));
    }
}

impl<'a> nohash_hasher::IsEnabled for PolygonVertexKey<'a> {}

// STATIC MAP DERIVED FROM DYNAMIC
#[derive(Default, Clone, Debug)]
pub struct StaticPolygonStorage{
    //pub positions : Vec<f32>,
    //pub normals : Vec<f32>,
    //pub uvmaps : Vec<UVMapData>,

    //pub pure_indices : Vec<u32>
    data : PolygonStorageData,
    max_index : usize,
}

impl StaticPolygonStorage{
    fn new(dynamic_data : DynamicPolygonStorage) -> StaticPolygonStorage{
        StaticPolygonStorage{
            data : dynamic_data.get_data().unwrap().clone(),
            max_index : dynamic_data.get_max_index()
        }
    }
}

impl PolygonStorageTrait for StaticPolygonStorage {
    fn get_data(&self) -> Option<&PolygonStorageData> {
        Some(&self.data)
    }
    fn get_type(&self) -> PolygonStorageType{
        PolygonStorageType::Static
    }
    
    // only useful for dynamic meshes
    fn gen_index_buffer(&mut self) {}
    fn gen_vertex_buffer(&mut self){}
    fn clear_index_buffer(&mut self){}
    fn clear_vertex_buffer(&mut self){}
}

// DYNAMIC MAP (suitable for soft bodies and new triangles on the fly)
#[derive(Default, Clone, Debug)]
pub struct DynamicPolygonStorage{
    pub positions : Vec<f32>,
    pub normals : Vec<f32>,
    pub uvmaps : Vec<UVMapData>,

    indices : Vec<u32>,

    data : PolygonStorageData,
    vbo_ready : bool,
    ibo_ready : bool,
    pub max_index : usize,
}

impl DynamicPolygonStorage{
    pub fn new(positions : Vec<f32>, normals : Vec<f32>, uvmaps : Vec<UVMapData>, indices : Vec<u32>) -> DynamicPolygonStorage{
        let mut output = DynamicPolygonStorage{
            data : PolygonStorageData::new(),
            max_index : 0,
            positions,
            normals,
            uvmaps,
            vbo_ready : false,
            ibo_ready : false,
            indices
        };
        output.regenerate_data();
        output.gen_vertex_buffer();
        output.gen_index_buffer();
        output
    }
    pub fn get_max_index(&self) -> usize {
        self.max_index
    }
    pub fn regenerate_data(&mut self){
        let mut vertex_buffer : Vec<PolygonVertexKey> =Vec::with_capacity(self.indices.len()/(2+self.uvmaps.len())/2);
        let mut index_buffer : IntMap<PolygonVertexKey, u32> = Default::default();
        for (id, original_data) in (&self.indices).chunks(2+self.uvmaps.len()).enumerate(){
            let polygon = PolygonVertexKey::new(original_data);
            if index_buffer.contains_key(&polygon){
                vertex_buffer.push(polygon);
                index_buffer.insert(polygon, id as u32);
            }
        }
        self.data.vertex_buffer_ = Vec::with_capacity(3*vertex_buffer.len());
        self.data.index_buffer_ = Vec::with_capacity(3*index_buffer.len());
        //TODO
    }
}


impl PolygonStorageTrait for DynamicPolygonStorage{
    fn get_data(&self) -> Option<&PolygonStorageData> {
        if self.vbo_ready && self.ibo_ready {
            Some(&self.data)
        }
        else {
            None
        }
    }
    fn get_type(&self) -> PolygonStorageType{
        PolygonStorageType::Dynamic
    }
    
    // only useful for dynamic meshes
    fn gen_index_buffer(&mut self) {}
    fn gen_vertex_buffer(&mut self){}
    fn clear_index_buffer(&mut self){}
    fn clear_vertex_buffer(&mut self){}
}