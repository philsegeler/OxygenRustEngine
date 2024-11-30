use super::dummy_structs::*;

pub trait WinsysBaseTrait{
    fn update_events_single_thread(&mut self);
    fn is_done(&self) -> bool;
    fn update_window(&self, update_info : WinsysUpdateInfo) -> WinsysOutput;
}

pub struct DummyWinsys;
impl WinsysBaseTrait for DummyWinsys{
    fn update_events_single_thread(&mut self){

    }
    fn is_done(&self) -> bool{
        true
    }
    fn update_window(&self, _ : WinsysUpdateInfo) -> WinsysOutput{
        Default::default()
    }
}