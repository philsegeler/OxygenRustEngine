use std::sync::atomic::{AtomicUsize, Ordering};
use crate::oe::math::{self as oe_math, DQuat};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub enum ObjectType {
    Mesh,
    Light,
    Camera,
    #[default]
    Custom
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CommonObjectData{
    pub id_ : usize,
    pub parent : usize,
    pub visible : bool,
    pub type_ : ObjectType,
    pub pos : [f64; 3],
    pub rot : [f64; 4],
    pub sca : [f64; 3],
    pub radius : f64,
    pub bbox_dims : [f64; 2]
}

impl CommonObjectData {
    pub fn new(type_ : ObjectType) -> CommonObjectData{
        static ID_COUNT : AtomicUsize = AtomicUsize::new(1);
        let mut objdata : CommonObjectData = Default::default();
        objdata.type_ = type_;
        objdata.id_ = ID_COUNT.fetch_add(1, Ordering::Relaxed);
        objdata
    }
}

pub trait ObjectTrait : Send + std::fmt::Debug {
    // trait to be defined functions
    fn get_data(&self) -> &CommonObjectData;
    fn get_data_mut(&mut self) -> &mut CommonObjectData;

    // trait functions with default automatic implementations
    fn id(&self) -> usize {
        self.get_data().id_
    }
    fn get_parent(&self) -> usize {
        self.get_data().parent
    }
    fn set_parent(&mut self, parent : usize) {
        self.get_data_mut().parent = parent;
    }
    fn get_visible(&self) -> bool {
        self.get_data().visible
    }
    fn set_visible(&mut self, visible : bool) {
        self.get_data_mut().visible = visible;
    }
    fn get_type(&self) -> ObjectType {
        self.get_data().type_
    }
    fn get_pos(&self) -> oe_math::DVec3 {
        let data = self.get_data();
        oe_math::DVec3::new(&data.pos)
    }
    fn set_pos(&mut self, pos : oe_math::DVec3) {
        let data = self.get_data_mut();
        data.pos = [pos[0], pos[1], pos[2]];
    }
    fn get_rot(&self) -> oe_math::DQuat {
        let data = self.get_data();
        oe_math::DQuat::new(&data.rot)
    }
    fn set_rot(&mut self, new_quat : oe_math::DQuat) {
        let data = self.get_data_mut();
        data.rot = [new_quat[0], new_quat[1], new_quat[2], new_quat[3]];
    }
    fn get_model_mat(&self) -> oe_math::DMat4x4 {
        let data = self.get_data();
        let mut model_mat = oe_math::DMat4x4::new_identity();
        let scale_vec = oe_math::DVec3::new(&data.sca);
        let translation_vec = oe_math::DVec3::new(&data.pos); 
        let rot_mat = DQuat::new(&data.rot).to_mat4x4();
        model_mat = rot_mat * oe_math::scale(model_mat.clone(), scale_vec);
        oe_math::translate(model_mat, translation_vec)
    }
    fn get_view_mat(&self) -> oe_math::DMat4x4 {
        let data = self.get_data();
        let mut view_mat = oe_math::DMat4x4::new_identity();
        let translation_vec = oe_math::DVec3::new(&data.pos); 
        let rot_mat = DQuat::new(&data.rot).to_mat4x4();
        view_mat = rot_mat * oe_math::translate(view_mat,translation_vec);
        view_mat
    }
    fn get_bbox_dimensions(&self) -> [f64; 2]{
        let data = self.get_data();
        data.bbox_dims
    }
    fn get_bounding_radius(&self) -> f64{
        let data = self.get_data();
        data.radius
    }
}