use std::sync::atomic::{Ordering, AtomicUsize};

#[derive(Default, Debug, Clone, Copy)]
pub struct Material {
    id_ : usize,
    pub dif_ : [f32; 4],
    pub scol : [f32; 3],
    pub alpha : f32,
    pub translucency : f32,
    pub illuminosity : f32,
    pub specular_intensity : f32,
    pub specular_hardness : f32,
}

impl Material{
    pub fn new() -> Material{
        static ID_COUNT : AtomicUsize = AtomicUsize::new(1);
        let mut output : Material = Default::default();
        output.id_ = ID_COUNT.fetch_add(1, Ordering::Relaxed);
        output
    }

    pub fn id(&self) -> usize{
        self.id_
    }
}