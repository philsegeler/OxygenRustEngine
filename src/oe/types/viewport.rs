use std::sync::atomic::{Ordering, AtomicUsize};

#[derive(Default)]
pub struct ViewPort {
    id_ : usize,
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