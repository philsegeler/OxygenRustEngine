use light::LightType;
use compact_str::CompactString;
use polygonstoragetrait::{UVMapData, VertexGroup};
use std::sync::{Arc, Mutex};
//use nohash_hasher::IntMap;

use super::parser::*;
use super::super::types::*;
use super::super::types::global_scenegraph::*;

#[derive(Default, Debug)]
pub struct Interpreter{
    pub objects_ : InterpreterElementWrapper<Box<dyn object_trait::ObjectTrait>>,
    pub polygons_ : InterpreterElementWrapper<Box<dyn polygonstoragetrait::PolygonStorageTrait>>,
    pub scenes_  : InterpreterElementWrapper<scene::Scene>,
    pub materials_  : InterpreterElementWrapper<material::Material>,
    //pub materials_strong : IntMap<usize, Arc<Mutex<(material::Material, bool)>>>,
    pub viewports_  : InterpreterElementWrapper<viewport::ViewPort>,
    pub world : Option<world::World>,
}

impl Interpreter{
    pub fn interpret(&mut self, input_str: &str){
        use std::time::Instant;
        let before = Instant::now();
        let element = parse_string(input_str);
        let after = Instant::now();
        println!("[Performance] Time parsing: {:?}", (after-before).as_secs_f64());

        let before = Instant::now();
        self.world = Some(self.process_world(&element));
        let after = Instant::now();
        println!("[Performance] Time interpreting: {:?}", (after-before).as_secs_f64());
        //println!("{:?}", self.world);
    }

    fn process_world(&mut self, element : &Element) -> world::World{
        let mut output: world::World = Default::default();

        for base_e in &element.elements_ref()["Scene"]{
            let scene = self.process_scene(&base_e.get().unwrap());
            let some_id;{
                some_id = scene.lock().unwrap().0.id();
            }
            let some_name = self.scenes_.get_name(&some_id).unwrap();
            output.scenes.insert(some_name.into(), scene.clone());
        }

        for base_e in element.elements_ref().get("ViewportConfig").unwrap_or(&Default::default()){
            let viewport = self.process_viewport(&base_e.get().unwrap());
            let some_id;{
                some_id = viewport.lock().unwrap().0.id();
            }
            let some_name = self.viewports_.get_name(&some_id).unwrap();
            output.viewports.insert(some_name.into(), viewport.clone());
        }
        
        let loaded_scene = element.assignments_ref()["loaded_scene"].get_str().unwrap();
        let loaded_viewport = element.assignments_ref()["loaded_viewport"].get_str().unwrap();

        output.loaded_scene = self.scenes_.get_id(loaded_scene).unwrap_or(0);
        output.loaded_viewport = self.viewports_.get_id(loaded_viewport).unwrap_or(0);

        output
    }

    fn process_scene(&mut self, element : &Element) -> Arc<Mutex<(scene::Scene, bool)>>{
        let output = Arc::new(Mutex::new((scene::Scene::new(), true)));
        let mut output_unlocked = output.lock().unwrap();
        
        for base_e in element.elements_ref().get("Material").unwrap_or(&Default::default()){
            let _ = self.process_material(&base_e.get().unwrap());
            //let some_id;{
            //    some_id = obj.lock().unwrap().id();
            //}
            //output_unlocked.materials.insert(some_id, Arc::downgrade(&obj));
        }
        for base_e in element.elements_ref().get("Camera").unwrap_or(&Default::default()){
            let obj = self.process_camera(&base_e.get().unwrap());
            let some_id;{
                some_id = obj.lock().unwrap().0.id();
            }
            let some_name = self.objects_.get_name(&some_id).unwrap();
            output_unlocked.0.objects.insert(some_name.into(), obj.clone());
        }
        for base_e in element.elements_ref().get("Light").unwrap_or(&Default::default()){
            let obj = self.process_light(&base_e.get().unwrap());
            let some_id;{
                some_id = obj.lock().unwrap().0.id();
            }
            let some_name = self.objects_.get_name(&some_id).unwrap();
            output_unlocked.0.objects.insert(some_name.into(), obj.clone());
        }
        for base_e in element.elements_ref().get("Mesh").unwrap_or(&Default::default()){
            let obj = self.process_mesh(&base_e.get().unwrap());
            let some_id;{
                some_id = obj.lock().unwrap().0.id();
            }
            let some_name = self.objects_.get_name(&some_id).unwrap();
            output_unlocked.0.objects.insert(some_name.into(), obj.clone());
        }

        let final_output = output.clone();
        self.scenes_.insert(output_unlocked.0.id(), output.clone(), element.attributes_ref()["name"].get_str().unwrap());
        final_output
    }

    fn process_camera(&mut self, element : &Element) -> Arc<Mutex<(Box<dyn object_trait::ObjectTrait>, bool)>>{

        let ar = element.assignments_ref()["aspect_ratio"].get_float().unwrap() as f32;
        let fov = element.assignments_ref()["fov"].get_float().unwrap() as f32;
        let near = element.assignments_ref()["near"].get_float().unwrap() as f32;
        let far = element.assignments_ref()["far"].get_float().unwrap() as f32;
        

        let output:  Arc<Mutex<(Box<dyn object_trait::ObjectTrait>, bool)>> = Arc::new(Mutex::new((Box::new(camera::Camera::new(ar, fov, near, far)), true)));
        let mut output_unlocked = output.lock().unwrap();

        let cs_v = element.assignments_ref()["current_state"].get_float_list().unwrap();
        let data = output_unlocked.0.get_data_mut();
        data.pos = [cs_v[0], cs_v[1], cs_v[2]];
        data.rot = [cs_v[3], cs_v[4], cs_v[5], cs_v[6]];
        data.sca = [cs_v[7], cs_v[8], cs_v[9]];
        
        let parent = element.assignments_ref()["parent"].get_str().unwrap();
        data.parent = self.objects_.get_id(parent).unwrap_or(0);
        
        let visible = element.attributes_ref()["visible"].get_int().unwrap();
        data.visible = visible != 0;

        let final_output = output.clone();
        self.objects_.insert(output_unlocked.0.id(), output.clone(), element.attributes_ref()["name"].get_str().unwrap());
        final_output
    }

    fn process_light(&mut self, element : &Element) -> Arc<Mutex<(Box<dyn object_trait::ObjectTrait>, bool)>>{

        let ltype = LightType::from(element.assignments_ref()["light_type"].get_int().unwrap());
        let fov = element.assignments_ref()["fov"].get_float().unwrap() as f32;
        let range = element.assignments_ref()["range"].get_float().unwrap() as f32;
        let intensity = element.assignments_ref()["intensity"].get_float().unwrap() as f32;
        

        let output: Arc<Mutex<(Box<dyn object_trait::ObjectTrait>, bool)>> = Arc::new(Mutex::new((Box::new(light::Light::new(ltype, intensity, fov, range)), true)));
        let mut output_unlocked = output.lock().unwrap();

        let cs_v = element.assignments_ref()["current_state"].get_float_list().unwrap();
        let data = output_unlocked.0.get_data_mut();
        data.pos = [cs_v[0], cs_v[1], cs_v[2]];
        data.rot = [cs_v[3], cs_v[4], cs_v[5], cs_v[6]];
        data.sca = [cs_v[7], cs_v[8], cs_v[9]];
        
        let parent = element.assignments_ref()["parent"].get_str().unwrap();
        data.parent = self.objects_.get_id(parent).unwrap_or(0);
        
        let visible = element.attributes_ref()["visible"].get_int().unwrap();
        data.visible = visible != 0;

        let final_output = output.clone();
        self.objects_.insert(output_unlocked.0.id(), output.clone(), element.attributes_ref()["name"].get_str().unwrap());
        final_output
    }

    fn process_mesh(&mut self, element : &Element) -> Arc<Mutex<(Box<dyn object_trait::ObjectTrait>, bool)>>{

        let name = element.attributes_ref()["name"].get_str().unwrap();
        let positions = element.assignments_ref()["vertices"].get_float_list().unwrap().iter().map(|x| *x as f32).collect();
        let normals = element.assignments_ref()["normals"].get_float_list().unwrap().iter().map(|x| *x as f32).collect();
        let mut vgroups : Vec<VertexGroup> = Default::default();
        let mut uvmaps : Vec<UVMapData> = Default::default();
        
        for base_e in element.elements_ref().get("VertexGroup").unwrap_or(&Default::default()){
            let obj = self.process_vgroup(&base_e.get().unwrap());
            vgroups.push(obj);
        }

        let mut num_of_uvs = 0;
        for base_e in element.elements_ref().get("UVMapData").unwrap_or(&Default::default()){
            let obj = self.process_uvmap_data(&base_e.get().unwrap());
            uvmaps.push(obj);
            num_of_uvs += 1;
        }

        let num_of_triangles = element.elements_ref().get("Triangle").unwrap_or(&Default::default()).len();
        let mut indices = Vec::with_capacity(num_of_triangles*(2+num_of_uvs));
        for base_e in element.elements_ref().get("Triangle").unwrap_or(&Default::default()){
            self.process_triangle(&base_e.get_triangle().unwrap(), &mut indices, num_of_uvs);
        }
        

        let output: Arc<Mutex<(Box<dyn object_trait::ObjectTrait>, bool)>> = Arc::new(Mutex::new((Box::new(mesh::Mesh::new_static(positions, normals, uvmaps, indices, vgroups, name)), true)));
        let mut output_unlocked = output.lock().unwrap();

        let cs_v = element.assignments_ref()["current_state"].get_float_list().unwrap();
        let data = output_unlocked.0.get_data_mut();
        data.pos = [cs_v[0], cs_v[1], cs_v[2]];
        data.rot = [cs_v[3], cs_v[4], cs_v[5], cs_v[6]];
        data.sca = [cs_v[7], cs_v[8], cs_v[9]];
        
        let parent = element.assignments_ref()["parent"].get_str().unwrap();
        data.parent = self.objects_.get_id(parent).unwrap_or(0);
        
        let visible = element.attributes_ref()["visible"].get_int().unwrap();
        data.visible = visible != 0;

        let final_output = output.clone();
        self.objects_.insert(output_unlocked.0.id(), output.clone(), name);
        final_output
    }

    fn process_vgroup(&mut self, element : &Element) -> VertexGroup{
        let mut output : VertexGroup = Default::default();

        output.name =  CompactString::new(element.attributes_ref()["name"].get_str().unwrap());
        output.polygons = element.assignments_ref()["polygons"].get_int_list().unwrap().iter().map(|x| *x as u32).collect();
        
        
        let material_name = element.assignments_ref()["material_id"].get_str().unwrap();
        let material_id = self.materials_.get_id(material_name).unwrap();
        output.material = Some((material_name.into(), self.materials_[material_id].clone()));
        
        output
    }

    fn process_material(&mut self, element : &Element) -> Arc<Mutex<(material::Material, bool)>>{
        let output: Arc<Mutex<(material::Material, bool)>> = Arc::new(Mutex::new((material::Material::new(), true)));
        let mut output_unlocked = output.lock().unwrap();
        
        let dif_r = element.assignments_ref()["dif_r"].get_float().unwrap() as f32;
        let dif_g = element.assignments_ref()["dif_g"].get_float().unwrap() as f32;
        let dif_b = element.assignments_ref()["dif_b"].get_float().unwrap() as f32;
        let dif_a = element.assignments_ref()["dif_a"].get_float().unwrap() as f32;

        let scol_r = element.assignments_ref()["scol_r"].get_float().unwrap() as f32;
        let scol_g = element.assignments_ref()["scol_g"].get_float().unwrap() as f32;
        let scol_b = element.assignments_ref()["scol_b"].get_float().unwrap() as f32;

        output_unlocked.0.dif_ = [dif_r, dif_g, dif_b, dif_a];
        output_unlocked.0.scol = [scol_r, scol_g, scol_b];
        output_unlocked.0.alpha = element.assignments_ref()["alpha"].get_float().unwrap() as f32;
        output_unlocked.0.translucency = element.assignments_ref()["translucency"].get_float().unwrap() as f32;
        output_unlocked.0.illuminosity = element.assignments_ref()["illuminosity"].get_float().unwrap() as f32;
        output_unlocked.0.specular_intensity = element.assignments_ref()["specular_intensity"].get_float().unwrap() as f32;
        output_unlocked.0.specular_hardness = element.assignments_ref()["specular_hardness"].get_float().unwrap() as f32;

        let final_output = output.clone();
        self.materials_.insert(output_unlocked.0.id(), output.clone(), element.attributes_ref()["name"].get_str().unwrap());
        final_output
    }

    fn process_viewport(&mut self, element : &Element) -> Arc<Mutex<(viewport::ViewPort, bool)>>{
        let output: Arc<Mutex<(viewport::ViewPort, bool)>> = Arc::new(Mutex::new((viewport::ViewPort::new(), true)));
        let mut output_unlocked = output.lock().unwrap();
        
        output_unlocked.0.split_screen_positions_ = element.assignments_ref()["split_screen_positions"].get_float_list().unwrap().iter().map(|x| *x as f32).collect(); 
        output_unlocked.0.layer_combine_modes_ = element.assignments_ref()["layer_combine_modes"].get_int_list().unwrap().iter().map(|x| *x as u32).collect(); 
        
        let cameras = element.assignments_ref()["cameras"].get_str_list().unwrap();

        for cam_name in cameras{
            output_unlocked.0.cameras_.push(cam_name.clone());
        }
        
        let final_output = output.clone();
        self.viewports_.insert(output_unlocked.0.id(), output.clone(), element.attributes_ref()["name"].get_str().unwrap());
        final_output
    }

    fn process_uvmap_data(&mut self, element : &Element) -> UVMapData{
        let mut output: UVMapData = Default::default();
        output.elements = element.assignments_ref()["elements"].get_float_list().unwrap().iter().map(|x| *x as f32).collect();
        output
    }

    fn process_triangle(&mut self, element : &TriangleElement, indices : &mut Vec<u32>, _num_of_uvs : usize){
        for i in 0..element.num_of_uvs as usize{
            indices.push(element.v1[i] as u32);
        }
        for i in 0..element.num_of_uvs as usize{
            indices.push(element.v2[i] as u32);
        }
        for i in 0..element.num_of_uvs as usize{
            indices.push(element.v3[i] as u32);
        }
    }

}

pub fn interpret(input_str : &str){
    let mut interpreter : Interpreter = Default::default();
    interpreter.interpret(input_str);
    //println!("{:?}", interpreter);
}

pub fn interpret_file(filename : &str){
    use std::fs;
    use std::time::Instant;
    let before = Instant::now();
    let input_str = fs::read_to_string(filename).unwrap();
    let after = Instant::now();
    println!("[Performance] Time reading from file: {:?} secs", (after-before).as_secs_f64());

    interpret(&input_str);
}