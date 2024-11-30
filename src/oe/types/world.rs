use std::sync::{Arc, Mutex};
use nohash_hasher::*;

use super::scene::*;
use super::viewport::*;

#[derive(Default)]
pub struct World{
    scenes : IntMap<usize, Arc<Mutex<Scene>>>,
    viewports : IntMap<usize, Arc<Mutex<ViewPort>>>,

    loaded_scene : usize,
    loaded_viewport : usize,
}
