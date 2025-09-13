use std::sync::{Arc, Mutex};
use compact_str::CompactString;
use nohash_hasher::IntSet;

use super::super::super::types::polygonstoragetrait::*;
use super::super::super::math::*;

#[derive(Clone, Debug)]
pub struct CommonRenderData{
    pub id : usize,
    pub changed : bool,
    pub has_init : bool,
    pub data : Vec<f32>
}

impl Default for CommonRenderData{
    fn default() -> Self {
        CommonRenderData { id: 0, changed: true, has_init: false, data: Default::default() }
    }
}

impl CommonRenderData{
    pub fn new(id : usize) -> CommonRenderData{
        let mut output: CommonRenderData = Default::default();
        output.id = id;
        output
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UniformBufferData{
    pub id : usize,
    pub offset : u32,
    pub size : u32,
}

impl UniformBufferData{
    pub fn new() -> UniformBufferData{
        UniformBufferData { id:0, offset: 0, size: 0}
    }
}

#[derive(Clone, Debug)]
pub struct CameraRenderData{
    pub common_data : CommonRenderData,
    pub perspective_mat : Mat4x4,
    pub perspective_view_mat : Mat4x4,
    pub view_mat : Mat4x4,
    pub model_mat : DMat4x4,

    pub near : f32,
    pub far : f32,

    pub ubo : UniformBufferData
}

impl CameraRenderData{
    pub fn new(id : usize) -> CameraRenderData{
        CameraRenderData { 
            common_data: CommonRenderData::new(id), 
            perspective_mat: Mat4x4::default(), 
            perspective_view_mat: Mat4x4::default(), 
            view_mat: Mat4x4::default(), 
            model_mat: DMat4x4::default(), 
            near: 0.0, 
            far: 0.0, 
            ubo: UniformBufferData::new()
        }
    }

    pub fn update_renderer_data(&mut self){
        
    }
}

#[derive(Clone, Debug)]
pub struct MaterialRenderData{
    pub common_data : CommonRenderData,
    pub ubo : UniformBufferData
}

#[derive(Clone, Debug)]
pub struct VertexGroupRenderData{
    pub common_data : CommonRenderData,
    pub bone_mat :Mat4x4,
    pub ibo : usize,
    pub material : (usize, CompactString),
    pub offset : u32,
    pub size : usize,
}

#[derive(Clone, Debug)]
pub struct MeshRenderData{
    pub common_data : CommonRenderData,
    pub model_mat : DMat4x4,
    pub uvmaps :u8,
    pub bones : u8,

    pub vbo : usize,
    pub vbo_size : usize,

    pub vao : usize,
    pub ubo : UniformBufferData,

    pub vao_initialized : bool,
    pub vao_input : Vec<u32>,

    pub max_vec : [f32 ; 3],
    pub min_vec : [f32 ; 3],

    pub mesh : Arc<Mutex<(Box<dyn PolygonStorageTrait>, bool)>>,
    pub vgroups : Vec<VertexGroupRenderData>
}

#[derive(Clone, Debug)]
pub struct PointLightRenderData{
    pub common_data : CommonRenderData,
    pub model_mat : DMat4x4,
    pub color : Vec3,
    pub intensity : f32,
    pub range : f32,
    pub ubo : UniformBufferData,
}

#[derive(Clone, Debug)]
pub struct DirectionalLightRenderData{
    pub common_data : CommonRenderData,
    pub model_mat : DMat4x4,
    pub color : DVec3,
    pub intensity : f32,
    pub ubo : UniformBufferData,
}

#[derive(Clone, Debug)]
pub struct SceneRenderData{
    pub common_data : CommonRenderData,
    pub cameras : IntSet<usize>,
    pub meshes : IntSet<usize>,
    pub dirlights : IntSet<usize>,
    pub pt_lights : IntSet<usize>,
    pub materials : IntSet<usize>,
}

impl SceneRenderData{
    pub fn new(id : usize)-> SceneRenderData{
        SceneRenderData { 
            common_data: CommonRenderData::new(id),
            cameras: Default::default(),
            meshes: Default::default(), 
            dirlights: Default::default(), 
            pt_lights: Default::default(), 
            materials: Default::default() 
        }
    }
}

#[derive(Debug)]
pub struct ViewportRenderData{
    pub common_data : CommonRenderData,
    pub layers_ : Vec<u32>,
    pub cameras_ : Vec<usize>,
    pub layer_combine_modes_ : Vec<u32>,
    pub split_screen_positions_ : Vec<f32>
}