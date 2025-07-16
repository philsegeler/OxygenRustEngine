use super::object_trait::*;
use super::light::*;
use super::mesh::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Camera {
    data_ : CommonObjectData,
    aspect_ratio : f32,
    fov : f32,
    near : f32,
    far : f32,
}

impl Camera {
    pub fn new(aspect_ratio : f32, fov : f32, near : f32, far : f32) -> Camera{
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
    fn get_camera(&self) -> Option<Camera> {Some(self.clone())}
    fn get_light(&self) -> Option<Light> {None}
    fn get_mesh(&self) -> Option<Mesh> {None}
    fn get_data(&self) -> &CommonObjectData {
        &self.data_
    }
    fn get_data_mut(&mut self) -> &mut CommonObjectData {
        &mut self.data_
    }
}