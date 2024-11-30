//use crate::oe::TraitWrapper;

//use std::collections::HashSet;
use super::polygonstoragetrait::*;
use nohash_hasher::IntMap;
use ordered_map;
//use std::ops::Index;

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
    fn regenerate_data(&mut self) {
        
    }
}

// DYNAMIC MAP (suitable for soft bodies and new triangles on the fly)
#[derive(Default, Clone, Debug)]
pub struct DynamicPolygonStorage{
    pub positions : Vec<f32>,
    pub normals : Vec<f32>,
    pub uvmaps : Vec<UVMapData>,

    indices : TriangleIndices,

    data : PolygonStorageData,
    regenerated_data : bool,
    pub max_index : usize,
}

impl DynamicPolygonStorage{
    pub fn new(positions : Vec<f32>, normals : Vec<f32>, uvmaps : Vec<UVMapData>, indices : Vec<u32>, vgroups : Vec<VertexGroup>) -> DynamicPolygonStorage{
        let mut output = DynamicPolygonStorage{
            data : PolygonStorageData::new(vgroups),
            max_index : 0,
            positions,
            normals,
            indices : TriangleIndices::new(indices, uvmaps.len()),
            uvmaps,
            regenerated_data : false,
            
        };
        output.regenerate_data();
        output
    }
    pub fn get_max_index(&self) -> usize {
        self.max_index
    }
}


impl PolygonStorageTrait for DynamicPolygonStorage{
    fn get_data(&self) -> Option<&PolygonStorageData> {
        if self.regenerated_data {
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
    fn regenerate_data(&mut self){
        use std::time;
        let mut before = time::Instant::now();
        let mut vertex_buffer : Vec<PolygonVertexKey> =Vec::with_capacity(self.indices.len()/(2+self.uvmaps.len())/2);
        let mut index_buffer : IntMap<PolygonVertexKey, u32> = Default::default();

        index_buffer.reserve(self.indices.len()/(2+self.uvmaps.len())/2);
        for original_data in (&self.indices.data).chunks(2+self.uvmaps.len()){
            
            let polygon = PolygonVertexKey::new(original_data);
            if !index_buffer.contains_key(&polygon){
                vertex_buffer.push(polygon);
                index_buffer.insert(polygon, index_buffer.len() as u32);
            }
        }
        let mut after = time::Instant::now();
        println!("Generate preliminaries {:?}", (after-before).as_secs_f64());


        before = time::Instant::now();
        self.data.num_of_uvs = self.uvmaps.len() as u8;
        let vbo_offset = 6+self.uvmaps.len()*2;
        self.data.vertex_buffer_ = Vec::with_capacity(vbo_offset*vertex_buffer.len());
        //self.data.vertex_buffer_.shrink_to_fit();

        //gen vertex buffer
        for vertex in vertex_buffer.iter() {
            //let final_id = id*vbo_offset;
            let init_pos = vertex[0] as usize*3;
            let init_nor = vertex[1] as usize*3;
            self.data.vertex_buffer_.extend_from_slice(&self.positions[init_pos..init_pos+3]);
            self.data.vertex_buffer_.extend_from_slice(&self.normals[init_nor..init_nor+3]);
        
            for (uv_id, uvmap) in self.uvmaps.iter().enumerate(){
                self.data.vertex_buffer_.push(uvmap.elements[vertex[2+uv_id] as usize*2]);
                self.data.vertex_buffer_.push(uvmap.elements[vertex[2+uv_id] as usize*2+1]);
            }
        }

        after = time::Instant::now();
        println!("gen vertex buffer {:?}", (after-before).as_secs_f64());
        
        before = time::Instant::now();
        //gen index buffer
        for (id, vgroup) in self.data.vgroups.iter().enumerate(){
            self.data.index_buffers_.insert(id, Vec::with_capacity(vgroup.polygons.len()*3));
            self.data.index_buffers_.get_mut(&id).unwrap().shrink_to_fit();
            //println!("{:?}", index_buffer);
            //println!("{:?}", vertex_buffer);
            for tri in vgroup.polygons.iter(){
                let final_id = *tri as usize;
                let triangle1 = PolygonVertexKey::new(&self.indices[(final_id, 0)]);
                let triangle2 = PolygonVertexKey::new(&self.indices[(final_id, 1)]);
                let triangle3 = PolygonVertexKey::new(&self.indices[(final_id, 2)]);
                //println!("{:?}", triangle1);
                //println!("{:?}", triangle2);
                //println!("{:?}", triangle3);
                self.data.index_buffers_.get_mut(&id).unwrap().push(index_buffer[&triangle1]);
                self.data.index_buffers_.get_mut(&id).unwrap().push(index_buffer[&triangle2]);
                self.data.index_buffers_.get_mut(&id).unwrap().push(index_buffer[&triangle3]);
            }
        }
        after = time::Instant::now();
        println!("gen index buffer {:?}", (after-before).as_secs_f64());
        self.regenerated_data = true;
    }
}

#[cfg(test)]
pub mod polygonstoragetest{
    use super::{DynamicPolygonStorage, UVMapData};
    use super::super::polygonstoragetrait::*;

    #[test]
    fn test_dynamic_polygon_storage(){
        let positions = vec![0.01, 0.02, 0.03,
                                            0.04, 0.05, 0.06,
                                            0.07, 0.08, 0.09,
                                            0.1, 0.11, 0.12,
                                            0.13, 0.14, 0.15,
                                            0.16, 0.17, 0.18];
        let normals = vec![1.01, 1.02, 1.03,
                                            1.04, 1.05, 1.06,
                                            1.07, 1.08, 1.09,
                                            1.1, 1.11, 1.12,
                                            1.13, 1.14, 1.15,
                                            1.16, 1.17, 1.18];
        let uvs1 = vec![2.01, 2.02,
                                            2.03, 2.04,
                                            2.05, 2.06,
                                            2.07,  2.08,
                                            2.09, 2.1,
                                            2.11, 2.12,];
        let uvs2 = vec![3.01, 3.02,
                                            3.03, 3.04,
                                            3.05, 3.06,
                                            3.07, 3.08,
                                            3.09, 3.1,
                                            3.11, 3.12,];
        let uvmaps = vec![UVMapData{elements : uvs1}, UVMapData{elements : uvs2}];
        let indices = vec![0, 0, 0, 0,
                                     1, 1, 1, 1,
                                     2, 2, 2, 2,
                                     1, 1, 1, 1, 
                                     2, 2, 2, 2,
                                     3, 3, 3, 3,
                                     0, 1, 2, 3,
                                     1, 2, 3, 4,
                                     2, 2, 2, 2];
        let vgroups = vec![VertexGroup{polygons : vec![0, 1, 2]}, VertexGroup{polygons : vec![0, 1]}];
        let dynamic_polygons = DynamicPolygonStorage::new(positions, normals, uvmaps, indices, vgroups);
        println!("{:?}", dynamic_polygons.get_vertex_buffer());
        println!("{:?}", dynamic_polygons.get_index_buffer(0));
        println!("{:?}", dynamic_polygons.get_index_buffer(1));
        let vbo_out = vec![0.01, 0.02, 0.03, 1.01, 1.02, 1.03, 2.01, 2.02, 3.01, 3.02, 0.04, 0.05, 0.06, 1.04, 1.05, 1.06, 2.03, 2.04, 3.03, 3.04, 0.07, 0.08, 0.09, 1.07, 1.08, 1.09, 2.05, 2.06, 3.05, 3.06, 0.1, 0.11, 0.12, 1.1, 1.11, 1.12, 2.07, 2.08, 3.07, 3.08, 0.01, 0.02, 0.03, 1.04, 1.05, 1.06, 2.05, 2.06, 3.07, 3.08, 0.04, 0.05, 0.06, 1.07, 1.08, 1.09, 2.07, 2.08, 3.09, 3.1];
        let ibo1_out = vec![0, 1, 2, 1, 2, 3, 4, 5, 2];
        let ibo2_out = vec![0, 1, 2, 1, 2, 3];

        assert!(*dynamic_polygons.get_vertex_buffer() == vbo_out);
        assert!(*dynamic_polygons.get_index_buffer(0) == ibo1_out);
        assert!(*dynamic_polygons.get_index_buffer(1) == ibo2_out);
    }
}

pub fn test_dynamic_polygon_storage_large(triangles_num : usize){
    use std::time;

    let positions: Vec<f32> = (0..triangles_num*3).map(|_| rand::random::<f32>()*2.0-1.0).collect();
    let normals: Vec<f32> = (0..triangles_num*3).map(|_| rand::random::<f32>()*2.0-1.0).collect();
    let uvs1: Vec<f32> = (0..triangles_num*2).map(|_| rand::random::<f32>()).collect();
    let uvs2: Vec<f32> = (0..triangles_num*2).map(|_| rand::random::<f32>()).collect();
    
    let uvmaps = vec![UVMapData{elements : uvs1}, UVMapData{elements : uvs2}];
    let indices: Vec<u32> = (0..triangles_num*6).map(|_| rand::random::<u32>()%(triangles_num as u32)).collect();
    let total_vgroups : Vec<u32> = (0..triangles_num*2).map(|_|rand::random::<u32>()%(triangles_num as u32/4)).collect();
    let vgroups = vec![VertexGroup{polygons : total_vgroups[0..triangles_num].to_owned()}, VertexGroup{polygons:total_vgroups[triangles_num..].to_owned()}];
    
    let before = time::Instant::now();
    let dynamic_polygons = DynamicPolygonStorage::new(positions, normals, uvmaps, indices, vgroups);
    let after = time::Instant::now();
    //println!("{:?}", dynamic_polygons.get_vertex_buffer());
    //println!("{:?}", dynamic_polygons.get_index_buffer(0));
    //println!("{:?}", dynamic_polygons.get_index_buffer(1));
    println!("DynamicPolygonTest {:?} triangles. Elapsed time {:?} seconds", triangles_num, (after-before).as_secs_f64());
}