use compact_str::CompactString;

use super::dummy_structs::*;
use super::types::globalscenegraphchanged::*;

pub trait WinsysBaseTrait{
    fn update_events_single_thread(&mut self);
    fn is_done(&self) -> bool;
    fn update_window(&mut self, update_info : WinsysUpdateInfo) -> WinsysOutput;
}

pub struct DummyWinsys;
impl WinsysBaseTrait for DummyWinsys{
    fn update_events_single_thread(&mut self){

    }
    fn is_done(&self) -> bool{
        true
    }
    fn update_window(&mut self, _ : WinsysUpdateInfo) -> WinsysOutput{
        Default::default()
    }
}


pub trait RendererBaseTrait : Send{
    fn update_single_thread(&mut self, update_info : RendererUpdateInfo, winsys_output : WinsysOutput);
    fn update_data(&mut self, data : GlobalScenegraphChanged);
    fn get_name(&self) -> CompactString;
}

pub struct DummyRenderer;
impl RendererBaseTrait for DummyRenderer{
    fn update_single_thread(&mut self, _ : RendererUpdateInfo, _ : WinsysOutput){

    }
    fn update_data(&mut self, _ : GlobalScenegraphChanged){

    }
    fn get_name(&self) -> CompactString{
        CompactString::new("DummyRenderer")
    }
}