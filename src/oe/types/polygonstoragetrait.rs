//use std::sync::{Arc, Mutex};
use std::sync::atomic::{Ordering, AtomicUsize};
//use std::sync::{Arc, Mutex};
//use super::material::Material;
use compact_str::CompactString;
use nohash_hasher::IntMap;
use std::ops::Index;
//use super::material::*;

#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub enum PolygonStorageType {
    Dynamic,
    #[default]
    Static,
    SoftBody,
}

// HELPER STRUCTS
#[derive(Default, Clone, Debug)]
pub struct UVMapData{
    pub elements : Vec<f32>
}

#[derive(Clone, Debug, Default)]
pub struct VertexGroup{
    pub name : CompactString,
    pub polygons : Vec<u32>,
    pub material : Option<CompactString>
}

// polygon vertex key
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolygonVertexKey<'a>{
    data : &'a[u32]
}

impl<'a> PolygonVertexKey<'a>{
    pub fn new(data : &'a[u32]) -> PolygonVertexKey<'a>{
        PolygonVertexKey{
            data
        }
    }
    pub fn to_owned(&self) -> Box<[u32]>{
        self.data.to_owned().into_boxed_slice()
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

#[derive(Clone, Default, Debug)]
pub struct TriangleIndices {
    pub data : Vec<u32>,
    num_of_uvs : usize,
}

impl TriangleIndices{
    pub fn new(data : Vec<u32>, num_of_uvs : usize) -> TriangleIndices{
        TriangleIndices{
            data,
            num_of_uvs: num_of_uvs as usize,
        }
    }
    pub fn len(&self) -> usize{
        self.data.len()
    }
}

impl Index<(usize, usize)> for TriangleIndices {
    type Output = [u32];
    fn index(&self, ids : (usize, usize)) -> &Self::Output {
        let offset = 2+self.num_of_uvs;
        let final_id = (ids.0*3 + ids.1)*offset;
        &self.data[final_id..(final_id + offset)]
    }
} 

// TRAIT BASE STRUCT
#[derive(Default, Clone, Debug)]
pub struct PolygonStorageData{
    id_ : usize,
    pub vertex_buffer_ : Vec<f32>,
    pub index_buffers_ : IntMap<usize, Vec<u32>>,
    pub num_of_uvs : u8,
    pub vgroups : Vec<VertexGroup>,
    pub max_index : usize,
}

impl PolygonStorageData{
    pub fn new(vgroups : Vec<VertexGroup>) -> PolygonStorageData{
        static ID_COUNT : AtomicUsize = AtomicUsize::new(1);
        let mut output : PolygonStorageData = Default::default();
        output.id_ = ID_COUNT.fetch_add(1, Ordering::Relaxed);
        output.vgroups = vgroups;
        output
    }
}

pub trait PolygonStorageTrait : Send + std::fmt::Debug{
    // functions to implement
    fn get_data(&self) -> Option<&PolygonStorageData>;
    fn get_type(&self) -> PolygonStorageType;
    fn regenerate_data(&mut self);

    // derived functions
    fn get_max_index(&self) -> Option<usize> {
        if self.get_data().unwrap().max_index == 0 {
            None
        }
        else {
            Some(self.get_data().unwrap().max_index)
        }
    }

    fn get_vgroups(&self) -> &Vec<VertexGroup>{
        &self.get_data().unwrap().vgroups
    }
    fn get_num_uvs(&self) -> u8{
        self.get_data().unwrap().num_of_uvs
    }
    fn id(&self) -> usize{
        self.get_data().unwrap().id_
    }
    fn get_vertex_buffer(&self) -> &Vec<f32>{
        &self.get_data().unwrap().vertex_buffer_
    }
    fn get_index_buffer(&self, id : usize) -> &Vec<u32>{
        &self.get_data().unwrap().index_buffers_[&id]
    }

    fn len(&self) -> usize{
        let total_length = self.get_vertex_buffer().len();
        total_length / (6+self.get_num_uvs() as usize*2)
    }
    fn get_16bit_index_buffer(&self, id : usize) -> Option<Vec<u16>>{
        if self.len() < 65536 {
            let ibo = self.get_index_buffer(id);
            Some(ibo.iter().map(|x: &u32|->u16 {*x as u16}).collect())
        }
        else {
            None
        }
    }
    fn get_positions(&self) -> Vec<f32>{
        let vertices = self.get_vertex_buffer();
        let vertex_length = self.get_num_uvs() as usize*2 + 6;
        let mut output = Vec::with_capacity(vertices.len()/vertex_length * 3);
        for index in 0..vertices.len()/vertex_length{
            output.push(vertices[index*vertex_length]);
            output.push(vertices[index*vertex_length+1]);
            output.push(vertices[index*vertex_length+2]);
        }
        output
    }
    fn get_normals(&self) -> Vec<f32>{
        let vertices = self.get_vertex_buffer();
        let vertex_length = self.get_num_uvs() as usize*2 + 6;
        let mut output = Vec::with_capacity(vertices.len()/vertex_length * 3);
        for index in 0..vertices.len()/vertex_length{
            output.push(vertices[index*vertex_length+3]);
            output.push(vertices[index*vertex_length+4]);
            output.push(vertices[index*vertex_length+5]);
        }
        output
    }
    fn get_uvs(&self, id : usize) -> Option<Vec<f32>>{
        let vertices = self.get_vertex_buffer();
        let vertex_length = self.get_num_uvs() as usize*2 + 6;
        if id as u8 >= self.get_num_uvs(){
            return None;
        }
        let mut output = Vec::with_capacity(vertices.len()/vertex_length * 2);
        for index in 0..vertices.len()/vertex_length{
            output.push(vertices[index*vertex_length+6 + id*2]);
            output.push(vertices[index*vertex_length+7 + id*2]);
        }
        Some(output)
    }
    fn get_vgroup_index_buffer(&self, id : usize) -> Vec<u32>{
        //let vgroup = &self.get_vgroups()[id];
        let ibo = self.get_index_buffer(id);
        /*let mut output = Vec::with_capacity(vgroup.polygons.len()*3);
        for index in &vgroup.polygons{
            let actual_id = *index as usize *3;
            output.push(ibo[actual_id]);
            output.push(ibo[actual_id+1]);
            output.push(ibo[actual_id+2]);
        }
        output*/
        ibo.clone()
    }
}
