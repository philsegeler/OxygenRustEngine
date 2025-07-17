use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use compact_str::CompactString;

use super::scene::*;
use super::viewport::*;

#[derive(Default, Debug, Clone)]
pub struct World{
    pub scenes : HashMap<CompactString, Arc<Mutex<(Scene, bool)>>>,
    pub viewports : HashMap<CompactString, Arc<Mutex<(ViewPort, bool)>>>,

    pub loaded_scene : usize,
    pub loaded_viewport : usize,
}
