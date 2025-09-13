use std::sync::atomic::{Ordering, AtomicUsize};
use compact_str::CompactString;
//use nohash_hasher::*;
use std::collections::HashSet;


//use super::object_trait::*;

#[derive(Default, Clone, Debug)]
pub struct Scene{
    id_ : usize,
    pub objects : HashSet<CompactString>,//HashMap<CompactString, Arc<Mutex<(Box<dyn ObjectTrait>, bool)>>>,
    pub materials : HashSet<CompactString>
}

impl Scene{
    pub fn new() -> Scene{
        static ID_COUNT : AtomicUsize = AtomicUsize::new(1);
        let mut output : Scene = Default::default();
        output.id_ = ID_COUNT.fetch_add(1, Ordering::Relaxed);
        output
    }

    pub fn id(&self) -> usize{
        self.id_
    }
}