#![allow(dead_code)]
pub type Vector2 = [f32; 2];
pub type Vector3 = [f32; 3];
pub type Vector4 = [f32; 4];

pub type Matrix4 = [[f32; 4]; 4];

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position   : Vector4,
    pub texture    : Vector3,
    pub texture_id : u32,
    pub normal     : Vector3,
}

type GLTuple4 = (f32, f32, f32, f32);
type GLTuple3 = (f32, f32, f32);
impl Vertex {
    pub fn from((v, vt, vn) : (GLTuple4, GLTuple3, GLTuple3)) -> Vertex {
        //TODO do mtl stuff and hook up texture ids
        Vertex { position   : [ v.0,  v.1,  v.2, v.3],
                 normal     : [vn.0, vn.1, vn.2],
                 texture    : [vt.0, vt.1, vt.2],
                 texture_id : 0 }
    }
}






