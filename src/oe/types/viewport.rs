use std::sync::atomic::{Ordering, AtomicUsize};
use compact_str::CompactString;

#[derive(Default, Debug, Clone)]
pub struct ViewPort {
    id_ : usize,
    pub layers_ : Vec<u32>,
    pub cameras_ : Vec<CompactString>,
    pub layer_combine_modes_ : Vec<u32>,
    pub split_screen_positions_ : Vec<f32>
}

impl ViewPort{
    pub fn new() -> ViewPort{
        static ID_COUNT : AtomicUsize = AtomicUsize::new(1);
        let mut output : ViewPort = Default::default();
        output.id_ = ID_COUNT.fetch_add(1, Ordering::Relaxed);
        output
    }

    pub fn id(&self) -> usize{
        self.id_
    }
}