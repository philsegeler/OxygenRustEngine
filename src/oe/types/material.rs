use std::sync::atomic::{Ordering, AtomicUsize};

#[derive(Default, Debug)]
pub struct Material {
    id_ : usize
}

impl Material{
    fn new() -> Material{
        static ID_COUNT : AtomicUsize = AtomicUsize::new(1);
        let mut output : Material = Default::default();
        output.id_ = ID_COUNT.fetch_add(1, Ordering::Relaxed);
        output
    }

    fn id(&self) -> usize{
        self.id_
    }
}