use std::sync::{Arc, Weak, Mutex, atomic::{Ordering, AtomicUsize}};
use nohash_hasher::*;

use super::material::*;
use super::object_trait::*;

#[derive(Default, Clone)]
pub struct Scene{
    id_ : usize,
    objects : IntMap<usize, Arc<Mutex<Box<dyn ObjectTrait>>>>,
    materials : IntMap<usize, Weak<Mutex<Material>>>,
}

pub struct SceneRenderData{
    objects_ : Vec<usize>,
    materials_ : Vec<usize>,
}

impl Scene{
    fn new() -> Scene{
        static ID_COUNT : AtomicUsize = AtomicUsize::new(1);
        let mut output : Scene = Default::default();
        output.id_ = ID_COUNT.fetch_add(1, Ordering::Relaxed);
        output
    }

    fn id(&self) -> usize{
        self.id_
    }
}