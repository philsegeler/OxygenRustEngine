

use compact_str::CompactString;

use super::super::dummy_structs::*;
use super::super::base_traits::*;
use super::super::types::globalscenegraphchanged::GlobalScenegraphChanged;


#[derive(Debug, Default)]
pub struct RendererCompat{
    new_data : Option<GlobalScenegraphChanged>,
    renderer_data : RendererUpdateInfo,
    winsys_data : WinsysOutput
}

impl RendererCompat{
    pub fn new() -> RendererCompat{
        Default::default()
    }
}

impl RendererBaseTrait for RendererCompat{
    fn update_single_thread(&mut self, updated_data : RendererUpdateInfo, winsys_data : WinsysOutput){
        self.renderer_data = updated_data;
        self.winsys_data = winsys_data;
    }
    fn update_data(&mut self, new_data : GlobalScenegraphChanged){
        self.new_data = Some(new_data);
        //println!("{:?}", &self.new_data);
    }
    fn get_name(&self) -> CompactString{
        CompactString::new("RendererCompat")
    }
}