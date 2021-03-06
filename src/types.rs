#![allow(dead_code)]
pub type Vector2 = [f32; 2];
pub type Vector3 = [f32; 3];
pub type Vector4 = [f32; 4];

pub type Matrix4 = [[f32; 4]; 4];
pub trait Matrix4Traits {
    fn view(position : Vector3, direction : Vector3, up : Vector3) -> Matrix4;
}

impl Matrix4Traits for Matrix4 {
    fn view(position : Vector3, direction : Vector3, up : Vector3) -> Matrix4{
        let f = {
            let f = direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };

        let s = [up[1] * f[2] - up[2] * f[1],
                 up[2] * f[0] - up[0] * f[2],
                 up[0] * f[1] - up[1] * f[0]];

        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]  
        };

        let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
                 f[2] * s_norm[0] - f[0] * s_norm[2],
                 f[0] * s_norm[1] - f[1] * s_norm[0]];

        let p =
            [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

        [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0]     , p[1], p[2], 1.0],
        ]
    }
}

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






