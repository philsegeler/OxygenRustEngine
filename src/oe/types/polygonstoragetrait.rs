use std::sync::{Arc, Mutex};
use std::sync::atomic::{Ordering, AtomicUsize};
use nohash_hasher::IntMap;

use super::material::*;

#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub enum PolygonStorageType {
    Dynamic,
    #[default]
    Static
}

#[derive(Default, Clone, Debug)]
pub struct UVMapData{
    pub elements : Vec<f32>
}

#[derive(Clone, Debug, Default)]
pub struct VertexGroup{
    pub polygons : Vec<u32>,
    pub material : (usize, Arc<Mutex<Material>>)
}

#[derive(Default, Clone, Debug)]
pub struct PolygonStorageData{
    id_ : usize,
    pub vertex_buffer_ : Vec<f32>,
    pub index_buffers_ : IntMap<usize, Vec<u32>>,
    pub num_of_uvs : u8,
    pub vgroups : Vec<VertexGroup>,
}
impl PolygonStorageData{
    pub fn new() -> PolygonStorageData{
        static ID_COUNT : AtomicUsize = AtomicUsize::new(1);
        let mut output : PolygonStorageData = Default::default();
        output.id_ = ID_COUNT.fetch_add(1, Ordering::Relaxed);
        output
    }
}

pub trait PolygonStorageTrait : Send {
    // functions to implement
    fn get_data(&self) -> Option<&PolygonStorageData>;
    fn get_type(&self) -> PolygonStorageType;

    fn gen_index_buffer(&mut self);
    fn gen_vertex_buffer(&mut self);

    fn clear_index_buffer(&mut self);
    fn clear_vertex_buffer(&mut self);

    // derived functions

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
    fn get_index_buffer(&self, id : &usize) -> &Vec<u32>{
        &self.get_data().unwrap().index_buffers_[id]
    }

    fn len(&self) -> usize{
        let total_length = self.get_vertex_buffer().len();
        total_length / (6+self.get_num_uvs() as usize*2)
    }
    fn get_16bit_index_buffer(&self, id : &usize) -> Option<Vec<u16>>{
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
        let ibo = self.get_index_buffer(&id);
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