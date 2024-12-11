use super::object_trait::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Camera {
    data_ : CommonObjectData,
    aspect_ratio : f32,
    fov : f32,
    near : f32,
    far : f32,
}

impl Camera {
    fn new(aspect_ratio : f32, fov : f32, near : f32, far : f32) -> Camera{
        Camera{
            data_ : CommonObjectData::new(ObjectType::Camera),
            aspect_ratio,
            fov,
            near,
            far
        }
    }
}

impl ObjectTrait for Camera {
    fn get_data(&self) -> &CommonObjectData {
        &self.data_
    }
    fn get_data_mut(&mut self) -> &mut CommonObjectData {
        &mut self.data_
    }
}