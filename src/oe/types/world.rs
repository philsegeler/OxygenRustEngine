//use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use compact_str::CompactString;

//use super::scene::*;
//use super::viewport::*;

#[derive(Default, Debug, Clone)]
pub struct World{
    pub scenes : HashSet<CompactString>,
    pub viewports : HashSet<CompactString>,

    pub loaded_scene : CompactString,
    pub loaded_viewport : CompactString,
}
