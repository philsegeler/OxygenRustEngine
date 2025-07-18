use super::object_trait::*;
use super::camera::*;
use super::mesh::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub enum LightType {
    #[default]
    Point = 0,
    Sun = 1,
    Lamp = 2,
    CustomLight = 3
}

impl From<i32> for LightType {
    fn from(value: i32) -> Self {
        match value {
            _ if value == LightType::Point as i32 => LightType::Point,
            _ if value == LightType::Sun as i32 => LightType::Sun,
            _ if value == LightType::Lamp as i32 => LightType::Lamp,
            _ if value == LightType::CustomLight as i32 => LightType::CustomLight,
            _ => LightType::CustomLight,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Light {
    data_ : CommonObjectData,
    type_ : LightType,
    intensity : f32,
    fov : f32,
    range : f32,
    priority : u32
}

impl Light {
    pub fn new(type_ : LightType, intensity : f32, fov : f32, range : f32) -> Light{
        Light{
            data_ : CommonObjectData::new(ObjectType::Light),
            type_,
            intensity,
            fov,
            range,
            priority : 0
        }
    }
}

impl ObjectTrait for Light {
    fn get_camera(&self) -> Option<Camera> {None}
    fn get_light(&self) -> Option<Light> {Some(self.clone())}
    fn get_mesh(&self) -> Option<Mesh> {None}
    fn get_data(&self) -> &CommonObjectData {
        &self.data_
    }
    fn get_data_mut(&mut self) -> &mut CommonObjectData {
        &mut self.data_
    }
}