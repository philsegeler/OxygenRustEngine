extern crate nalgebra_glm as glm;

#[repr(transparent)]
#[derive(Clone, Debug, Default)]
pub struct DVec4 {
    data : glm::DVec4,
}

impl DVec4 {
    pub fn new_from_value(value : f64) -> DVec4 {
        DVec4 {data : glm::DVec4::new(value, value, value, value)}
    }
    pub fn new(data : &[f64; 4]) -> DVec4 {
        DVec4{data : glm::make_vec4(data)}
    }

    pub fn new3dim(x : f64, y : f64, z : f64) -> DVec4 {
        DVec4 {data : glm::DVec4::new(x, y, z, 1.0)}
    }

    pub fn get_f32_vec(&self) -> Vec<f32> {
        vec![self[0] as f32, self[1] as f32, self[2] as f32, self[3] as f32]
    }

}

impl std::ops::Index<usize> for DVec4
{
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => {&self.data.x}
            1 => {&self.data.y}
            2 => {&self.data.z}
            3 => {&self.data.w}
            _ => {panic!("Invalid index");}
        }
    }
}
impl std::ops::IndexMut<usize> for DVec4
{
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => {&mut self.data.x}
            1 => {&mut self.data.y}
            2 => {&mut self.data.z}
            3 => {&mut self.data.w}
            _ => {panic!("Invalid index");}
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Debug, Default)]
pub struct Vec4 {
    data : glm::Vec4,
}

impl Vec4 {
    pub fn new_from_value(value : f32) -> Vec4 {
        Vec4 {data : glm::Vec4::new(value, value, value, value)}
    }
    pub fn new(data : &[f32; 4]) -> Vec4 {
        Vec4{data : glm::make_vec4(data)}
    }

    pub fn new3dim(x : f32, y : f32, z : f32) -> Vec4 {
        Vec4 {data : glm::Vec4::new(x, y, z, 1.0)}
    }

    pub fn get_f32_vec(&self) -> Vec<f32> {
        vec![self[0] as f32, self[1] as f32, self[2] as f32, self[3] as f32]
    }

}

impl std::ops::Index<usize> for Vec4
{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => {&self.data.x}
            1 => {&self.data.y}
            2 => {&self.data.z}
            3 => {&self.data.w}
            _ => {panic!("Invalid index");}
        }
    }
}
impl std::ops::IndexMut<usize> for Vec4
{
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => {&mut self.data.x}
            1 => {&mut self.data.y}
            2 => {&mut self.data.z}
            3 => {&mut self.data.w}
            _ => {panic!("Invalid index");}
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Debug, Default)]
pub struct DVec3 {
    data : glm::DVec3,
}
impl DVec3 {
    pub fn new(data : &[f64; 3]) -> DVec3 {
        DVec3{data : glm::make_vec3(data)}
    }
    pub fn new3dim(x : f64, y : f64, z : f64) -> DVec3 {
        DVec3 {data : glm::DVec3::new(x, y, z)}
    }
}
impl std::ops::Index<usize> for DVec3
{
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => {&self.data.x}
            1 => {&self.data.y}
            2 => {&self.data.z}
            _ => {panic!("Invalid index");}
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Debug, Default)]
pub struct Vec3 {
    data : glm::Vec3,
}
impl Vec3 {
    pub fn new(data : &[f32; 3]) -> Vec3 {
        Vec3{data : glm::make_vec3(data)}
    }
    pub fn new3dim(x : f32, y : f32, z : f32) -> Vec3 {
        Vec3 {data : glm::Vec3::new(x, y, z)}
    }
}
impl std::ops::Index<usize> for Vec3
{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => {&self.data.x}
            1 => {&self.data.y}
            2 => {&self.data.z}
            _ => {panic!("Invalid index");}
        }
    }
}

// MATRIX 4x4
#[repr(transparent)]
#[derive(Clone, Debug, Default)]
pub struct DMat4x4 {
    data : glm::DMat4,
}

impl DMat4x4 {
    pub fn new(data : &[f64; 16]) -> DMat4x4 {
        DMat4x4{data : glm::make_mat4x4(data)}
    }
    pub fn new_from_value(val : f64) -> DMat4x4 {
        DMat4x4 {
            data : glm::make_mat4x4(&[val; 16])
        }
    }
    pub fn new_identity() -> DMat4x4 {
        DMat4x4 {
            data : glm::DMat4x4::identity()
        }
    }
    pub fn get_f32_vec(&self) -> Vec<f32> {
        glm::value_ptr(&glm::convert(self.data)).to_owned()
    }

}

impl std::ops::Mul<DMat4x4> for DMat4x4 {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self{data : self.data * rhs.data}
    }
}

impl std::ops::Mul<DVec4> for DMat4x4 {
    // The multiplication of rational numbers is a closed operation.
    type Output = DVec4;

    fn mul(self, rhs: DVec4) -> DVec4 {
        DVec4{data : self.data * rhs.data}
    }
}

#[repr(transparent)]
#[derive(Clone, Debug, Default)]
pub struct Mat4x4 {
    data : glm::Mat4,
}

impl Mat4x4 {
    pub fn new(data : &[f32; 16]) -> Mat4x4 {
        Mat4x4{data : glm::make_mat4x4(data)}
    }
    pub fn new_from_value(val : f32) -> Mat4x4 {
        Mat4x4 {
            data : glm::make_mat4x4(&[val; 16])
        }
    }
    pub fn new_identity() -> Mat4x4 {
        Mat4x4 {
            data : glm::Mat4x4::identity()
        }
    }
    pub fn get_f32_vec(&self) -> Vec<f32> {
        glm::value_ptr(&glm::convert(self.data)).to_owned()
    }

}

impl std::ops::Mul<Mat4x4> for Mat4x4 {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self{data : self.data * rhs.data}
    }
}

impl std::ops::Mul<Vec4> for Mat4x4 {
    // The multiplication of rational numbers is a closed operation.
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4{data : self.data * rhs.data}
    }
}

impl std::ops::Index<(usize, usize)> for Mat4x4{
    type Output = f32;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        & self.data[index]
    }
}
impl std::ops::IndexMut<(usize, usize)> for Mat4x4{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// QUATERNION 
#[repr(transparent)]
#[derive(Clone, Default)]
pub struct DQuat {
    data : glm::DQuat,
}
impl DQuat {
    pub fn new(data : &[f64; 4]) -> DQuat {
        DQuat{
            data : glm::make_quat(data)
        }
    }

    pub fn to_mat4x4(&self) -> DMat4x4 {
        DMat4x4{
            data : glm::quat_to_mat4(&self.data)
        }
    }

    pub fn get_f32_vec(&self) -> Vec<f32> {      
        vec![self[0] as f32, self[1] as f32, self[2] as f32, self[3] as f32]
    }

}

impl std::ops::Mul<DQuat> for DQuat {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self{data : self.data * rhs.data}
    }
}

impl std::ops::Index<usize> for DQuat
{
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => {&self.data.coords.x}
            1 => {&self.data.coords.y}
            2 => {&self.data.coords.z}
            3 => {&self.data.w}
            _ => {panic!("Invalid index");}
        }
    }
}
impl std::ops::IndexMut<usize> for DQuat
{
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => {&mut self.data.coords.x}
            1 => {&mut self.data.coords.y}
            2 => {&mut self.data.coords.z}
            3 => {&mut self.data.w}
            _ => {panic!("Invalid index");}
        }
    }
}

pub fn scale(mat_in : DMat4x4, vec_in : DVec3) -> DMat4x4{
    DMat4x4{
        data : glm::scale(&mat_in.data, &vec_in.data)
    }
}

pub fn translate(mat_in : DMat4x4, vec_in : DVec3) -> DMat4x4{
    DMat4x4{
        data : glm::translate(&mat_in.data, &vec_in.data)
    }
}

pub fn perspective(fov : f32, aspect : f32, near : f32, far : f32) -> Mat4x4{
    let fov_rad = fov;
    let range = (fov_rad/2.0).tan() * near;

    let sx = (2.0 * near) / (range*aspect + range*aspect);
    let sy = near/range;
    let sz = -(far + near)/ (far - near);
    let pz = -(2.0*far*near)/(far - near);

    let mut output = Mat4x4::new_from_value(0.0);
    output[(0, 0)] = sx;
    output[(1, 1)] = sy;
    output[(2, 2)] = sz;
    output[(3, 2)] = pz;
    output[(2, 3)] = -1.0;

    output
}