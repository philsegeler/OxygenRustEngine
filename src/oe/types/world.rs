use std::sync::{Arc, Mutex};
use nohash_hasher::*;

use super::scene::*;
use super::viewport::*;

#[derive(Default, Debug)]
pub struct World{
    pub scenes : IntMap<usize, Arc<Mutex<Scene>>>,
    pub viewports : IntMap<usize, Arc<Mutex<ViewPort>>>,

    pub loaded_scene : usize,
    pub loaded_viewport : usize,
}
