use crate::oe::math::Mat4x4;

use super::super::super::types::globalscenegraphchanged::GlobalScenegraphChanged;

use super::super::super::types::viewport::ViewPort;
use super::super::super::types::scene::*;
use super::super::super::types::mesh::*;
use super::super::super::types::material::*;
use super::super::super::types::camera::*;
use super::super::super::types::light::*;
use super::super::super::math;
use super::super::super::types::object_trait::{ChangedObjectEnum, ObjectTrait};
use super::super::super::types::polygonstoragetrait::VertexGroup;
use super::render_datacontainer::RenderDataContainer;

use super::render_data::*;

#[derive(Debug)]
pub struct DataHandler{
    pub cameras : RenderDataContainer<CameraRenderData>,
    pub materials : RenderDataContainer<MaterialRenderData>,
    pub meshes : RenderDataContainer<MeshRenderData>,

    pub dir_lights : RenderDataContainer<DirectionalLightRenderData>,
    pub pt_lights : RenderDataContainer<PointLightRenderData>,

    pub scenes : RenderDataContainer<SceneRenderData>,
    pub viewports : RenderDataContainer<ViewportRenderData>,

    pub has_dir_lights_changed : bool,
    pub has_pt_lights_changed : bool,
    pub load_spheres_or_bboxes : bool,
    pub loaded_viewport : usize
}

impl DataHandler{

    pub fn new() -> DataHandler{
        DataHandler { 
            cameras:RenderDataContainer::new(), 
            materials: RenderDataContainer::new(), 
            meshes: RenderDataContainer::new(), 
            dir_lights: RenderDataContainer::new(), 
            pt_lights: RenderDataContainer::new(), 
            scenes: RenderDataContainer::new(), 
            viewports: RenderDataContainer::new(), 
            has_dir_lights_changed: false, 
            has_pt_lights_changed: false, 
            load_spheres_or_bboxes: false, 
            loaded_viewport: 0 }
    }

    pub fn update(&mut self, restart_renderer : bool, load_minmax_elements : bool, elements : &GlobalScenegraphChanged) {
        
        self.scenes.update(restart_renderer);
        self.cameras.update(restart_renderer);
        self.dir_lights.update(restart_renderer);
        self.materials.update(restart_renderer);
        self.pt_lights.update(restart_renderer);
        self.meshes.update(restart_renderer);
        self.viewports.update(restart_renderer);

        self.load_spheres_or_bboxes = load_minmax_elements;
        let mut camera_ids: Vec<usize> = vec![];

        if !elements.is_empty(){
            println!("{:?}", elements);
            println!("RUNS DATA HANDLER");
        }

        // first handle materials
        for (id, name, material) in elements.materials_.get_data(){
            self.handle_material_data(id, material, name, &elements);
        }
        self.has_dir_lights_changed = false;
        self.has_pt_lights_changed = false;

        // then meshes and lights
        for (id, name, obj) in elements.objects_.get_data(){
            match obj{
                ChangedObjectEnum::Light(light) =>{self.handle_light_data(id, light, name, &elements);}
                ChangedObjectEnum::Mesh(mesh) =>{self.handle_mesh_data(id, mesh, name, &elements);}
                _ => {}
            }
        }
        // then cameras
        for (id, name, obj) in elements.objects_.get_data(){
            match obj{
                ChangedObjectEnum::Camera(camera) =>{
                    self.handle_camera_data(id, camera, name, &elements);
                    camera_ids.push(*id);
                }
                _ => {}
            }
        }

        // then scenes
        for (id, name, scene) in elements.scenes_.get_data(){
            self.handle_scene_data(id, scene, name, &elements);
        }
        // then viewport
        for (id, name, viewport) in elements.viewports_.get_data(){
            let loaded_viewport = &elements.world_.as_ref().unwrap().loaded_viewport;
            self.loaded_viewport = self.viewports.get_id(loaded_viewport).unwrap();
            self.handle_viewport_data(id, viewport, name, &elements);
        }

        for name in elements.materials_.get_deleted(){
            self.materials.remove_by_name(name);
        }
        for name in elements.scenes_.get_deleted(){
            self.scenes.remove_by_name(name);
        }
        for name in elements.objects_.get_deleted(){
            if self.meshes.contains_name(name){
                self.meshes.remove_by_name(name);
            }
            else if self.cameras.contains_name(name){
                self.cameras.remove_by_name(name);
            }
            else if self.pt_lights.contains_name(name){
                self.pt_lights.remove_by_name(name);
                self.has_pt_lights_changed = true;
            }
            else if self.dir_lights.contains_name(name){
                self.dir_lights.remove_by_name(name);
                self.has_dir_lights_changed = true;
            }
        }

        for name in elements.viewports_.get_deleted(){
            let id_opt = self.viewports.get_id(name);
            if let Some(id) = id_opt{
                if self.loaded_viewport == id{
                    self.loaded_viewport = 0;
                }
            }
            self.viewports.remove_by_name(name);
        }
        if !elements.is_empty(){
            println!("{:?}", self);
        }
    }

    ////////// HANDLE ELEMENTS ////////////////
    
    fn handle_mesh_data(&mut self, id : &usize, mesh : &Mesh, name : &str, _elements : &GlobalScenegraphChanged){
        
        let mut mesh_render_data: MeshRenderData;
        if self.meshes.contains(id){
            mesh_render_data = self.meshes[*id].clone();
            mesh_render_data.model_mat = mesh.get_model_mat();
            let polygons_unlocked = mesh.get_polygonstorage_unlocked();
            if polygons_unlocked.1{
                mesh_render_data.vgroups = self.handle_vgroup_data(polygons_unlocked.0.get_vgroups());
                mesh_render_data.mesh = mesh.polygon_storage_.1.clone();
                mesh_render_data.vao_initialized = false;
            }
        }
        else{
            let polygons_unlocked = mesh.get_polygonstorage_unlocked();
            mesh_render_data = MeshRenderData { 
                common_data: CommonRenderData::new(*id), 
                model_mat: mesh.get_model_mat(), 
                uvmaps: polygons_unlocked.0.get_num_uvs(), 
                bones: 0, 
                vbo: 0, 
                vbo_size: 0, 
                vao: 0, 
                ubo: UniformBufferData::new(), 
                vao_initialized: false, 
                vao_input: Default::default(), 
                max_vec: Default::default(), 
                min_vec: Default::default(), 
                mesh: mesh.polygon_storage_.1.clone(), 
                vgroups: self.handle_vgroup_data(polygons_unlocked.0.get_vgroups()),
            };
            drop(polygons_unlocked);
        }
        mesh_render_data.common_data.changed = true;
        self.meshes.insert(*id, mesh_render_data, name);
    }

    fn handle_material_data(&mut self, id : &usize, material : &Material, name : &str, _elements : &GlobalScenegraphChanged){
        
        let mut material_render_data: MaterialRenderData;
        if self.materials.contains(id){
            material_render_data = self.materials[*id].clone();
        }
        else{
            material_render_data = MaterialRenderData { 
                common_data: CommonRenderData::new(*id), 
                ubo: UniformBufferData::new()
            }
        }
        material_render_data.common_data.data = material.get_renderer_data();
        material_render_data.common_data.changed = true;
        self.materials.insert(*id, material_render_data, name);
    }

    fn handle_camera_data(&mut self, id : &usize, camera : &Camera, name : &str, _elements : &GlobalScenegraphChanged){
        
        let mut camera_render_data: CameraRenderData;
        let view_mat_64 = camera.get_view_mat().get_f32_vec();
        let view_mat = math::Mat4x4::new(&view_mat_64.try_into().unwrap());
        let perspective_mat = camera.get_perspective_mat();
        let perspective_view_mat = perspective_mat.clone()*view_mat.clone();

        if self.cameras.contains(id){
            camera_render_data = self.cameras[*id].clone();
        }
        else{
            camera_render_data = CameraRenderData::new(*id);
        }

        camera_render_data.perspective_mat = perspective_mat;
        camera_render_data.view_mat = view_mat;
        camera_render_data.perspective_view_mat = perspective_view_mat;
        camera_render_data.near = camera.near;
        camera_render_data.far = camera.far;

        camera_render_data.update_renderer_data();
        camera_render_data.common_data.changed = true;
        self.cameras.insert(*id, camera_render_data, name);
    }

    fn handle_light_data(&mut self, id : &usize, light : &Light, name : &str, _elements : &GlobalScenegraphChanged){
        match light.get_type(){
            LightType::Point => {
                
                let mut light_render_data;
                if self.pt_lights.contains(id) {
                    light_render_data = self.pt_lights[*id].clone();
                    light_render_data.model_mat = light.get_model_mat();
                    light_render_data.color = math::Vec3::new(&light.color);
                    light_render_data.intensity = light.intensity;
                    light_render_data.range = light.range;
                    light_render_data.common_data.changed = true;
                }
                else{
                    light_render_data = PointLightRenderData { 
                        common_data: CommonRenderData::new(*id), 
                        model_mat: light.get_model_mat(), 
                        color: math::Vec3::new(&light.color), 
                        intensity: light.intensity, 
                        range: light.range, 
                        ubo: UniformBufferData::new()
                    }
                }
                self.has_pt_lights_changed = true;
                self.pt_lights.insert(*id, light_render_data, name);
            }
            _ => {}
        }
    }

    fn handle_vgroup_data(&mut self, vgroups : &Vec<VertexGroup>) -> Vec<VertexGroupRenderData>{
        let mut output = Vec::with_capacity(vgroups.len());
        for vgroup in vgroups{
            output.push(VertexGroupRenderData { 
                common_data: CommonRenderData::new(0), 
                bone_mat: Mat4x4::new_identity(), ibo: 0, 
                material: (self.materials.get_id(vgroup.material.as_ref().unwrap()).unwrap(), vgroup.material.clone().unwrap()), 
                offset: 0, size: 0 
            })
        }
        output
    }

    fn handle_scene_data(&mut self, id : &usize, scene : &Scene, name : &str, _elements : &GlobalScenegraphChanged){
        let mut scene_render_data = SceneRenderData::new(*id);

        for elem in &scene.materials{
            scene_render_data.materials.insert(self.materials.get_id(&elem).unwrap());
        }
        for name in &scene.objects{
            if self.cameras.contains_name(name){
                scene_render_data.cameras.insert(self.cameras.get_id(name).unwrap());
            }
            else if self.meshes.contains_name(name){
                scene_render_data.meshes.insert(self.meshes.get_id(name).unwrap());
            }
            else if self.pt_lights.contains_name(name){
                scene_render_data.pt_lights.insert(self.pt_lights.get_id(name).unwrap());
            }
            else if self.dir_lights.contains_name(name){
                scene_render_data.dirlights.insert(self.dir_lights.get_id(name).unwrap());
            }
            else {
                //panic!("wha is happening '{:?}'", name);
            }
        }
        self.scenes.insert(*id, scene_render_data, name);
    }

    fn handle_viewport_data(&mut self, id : &usize, viewport : &ViewPort, name : &str, _elements : &GlobalScenegraphChanged){
        let viewport_render_data : ViewportRenderData = ViewportRenderData { 
            common_data: CommonRenderData::new(*id), 
            layers_: viewport.layers_.clone(), 
            cameras_: viewport.cameras_.iter().map(|x| self.cameras.get_id(x).unwrap()).collect(), 
            layer_combine_modes_: viewport.layer_combine_modes_.clone(), 
            split_screen_positions_: viewport.split_screen_positions_.clone() 
        };
        self.viewports.insert(*id, viewport_render_data, name);
    }
    /*fn delete_camera(&mut self, name : &str, _elements : &GlobalScenegraphChanged){
        //TODO
    }
    fn delete_material(&mut self, name : &str, _elements : &GlobalScenegraphChanged){
        //TODO
    }
    fn delete_mesh(&mut self, name : &str, _elements : &GlobalScenegraphChanged){
        
    }*/
}