use super::object_trait::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub enum LightType {
    Point,
    Sun,
    Lamp,
    #[default]
    CustomLight
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
    fn new(type_ : LightType, intensity : f32, fov : f32, range : f32) -> Light{
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
    fn get_data(&self) -> &CommonObjectData {
        &self.data_
    }
    fn get_data_mut(&mut self) -> &mut CommonObjectData {
        &mut self.data_
    }
}