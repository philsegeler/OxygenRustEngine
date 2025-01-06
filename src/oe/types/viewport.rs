use std::sync::atomic::{Ordering, AtomicUsize};

#[derive(Default)]
pub struct ViewPort {
    id_ : usize,
    layers_ : Vec<u32>,
    cameras_ : Vec<usize>,
    layer_combine_modes_ : Vec<u32>,
    split_screen_positions_ : Vec<f32>
}

impl ViewPort{
    fn new() -> ViewPort{
        static ID_COUNT : AtomicUsize = AtomicUsize::new(1);
        let mut output : ViewPort = Default::default();
        output.id_ = ID_COUNT.fetch_add(1, Ordering::Relaxed);
        output
    }

    fn id(&self) -> usize{
        self.id_
    }
}