use polygonstoragetrait::{TriangleIndices, UVMapData, VertexGroup};

use super::parser::*;
use super::super::types::*;
use super::super::types::global_variables::*;

#[derive(Default)]
struct Interpreter{
    objects_ : ElementWrapper<Box<dyn object_trait::ObjectTrait>>,
    polygons_ : ElementWrapper<Box<dyn polygonstoragetrait::PolygonStorageTrait>>,
    scenes_  : ElementWrapper<scene::Scene>,
    materials_  : ElementWrapper<material::Material>,
    viewports_  : ElementWrapper<viewport::ViewPort>,
}

impl Interpreter{
    pub fn interpret(&mut self, input_str: &str){

    }
    fn process_world(&mut self, element : &Element) -> world::World{
        let output = Default::default();
        output
    }
    fn process_scene(&mut self, element : &Element) -> scene::Scene{
        let output = Default::default();
        output
    }
    fn process_camera(&mut self, element : &Element) -> camera::Camera{
        let output = Default::default();
        output
    }
    fn process_light(&mut self, element : &Element) -> light::Light{
        let output = Default::default();
        output
    }
    fn process_mesh(&mut self, element : &Element) -> mesh::Mesh{
        let output = mesh::Mesh::new();
        output
    }
    fn process_vgroup(&mut self, element : &Element) -> VertexGroup{
        let output = Default::default();
        output
    }
    fn process_material(&mut self, element : &Element) -> material::Material{
        let output = Default::default();
        output
    }
    fn process_viewport(&mut self, element : &Element) -> viewport::ViewPort{
        let output = Default::default();
        output
    }
    fn process_uvmap_data(&mut self, element : &Element) -> UVMapData{
        let output = Default::default();
        output
    }
    fn process_triangle(&mut self, element : &Element) -> TriangleIndices{
        let output = Default::default();
        output
    }
}

pub fn interpret(input_str : &str){
    let mut interpreter : Interpreter = Default::default();
    interpreter.interpret(input_str);
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