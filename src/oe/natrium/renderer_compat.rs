

use compact_str::CompactString;

use super::super::dummy_structs::*;
use super::super::base_traits::*;
use super::super::types::globalscenegraphchanged::GlobalScenegraphChanged;
use super::datahandler::datahandler::*;

#[derive(Debug)]
pub struct RendererCompat{
    new_data : Option<GlobalScenegraphChanged>,
    renderer_update_info : RendererUpdateInfo,
    winsys_data : WinsysOutput,
    data : DataHandler
}

impl RendererCompat{
    pub fn new() -> RendererCompat{
        RendererCompat { 
            new_data: None, 
            renderer_update_info: Default::default(), 
            winsys_data:  Default::default(), 
            data : DataHandler::new()
        }
    }
}

impl RendererBaseTrait for RendererCompat{
    fn update_single_thread(&mut self){
        self.data.update(self.renderer_update_info.restart_renderer, self.renderer_update_info.render_bounding_boxes || self.renderer_update_info.render_bounding_spheres);

    }
    fn update_data(&mut self, new_data : GlobalScenegraphChanged, update_info : RendererUpdateInfo, winsys_output : WinsysOutput){
        self.data.set_changed(new_data);
        self.renderer_update_info = update_info;
        self.winsys_data = winsys_output;
        //println!("{:?}", &self.new_data);
    }
    fn get_name(&self) -> CompactString{
        CompactString::new("RendererCompat")
    }
}