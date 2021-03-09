pub mod math;
pub use math::*;

pub mod matrix4;
pub use matrix4::*;

pub mod vertex;
pub use vertex::*;

pub mod model;
pub use model::*;


pub type Maybe<T> = Result<T, Box<dyn std::error::Error>>;

#[allow(dead_code)]
pub type Vector2 = [f32; 2];
pub type Vector3 = [f32; 3];
pub type Vector4 = [f32; 4];

pub type Matrix4 = [[f32; 4]; 4];

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position   : Vector4,
    pub uvw        : Vector3,
    pub normal     : Vector3,
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices : Vec<Vertex>,
    pub indices  : Vec<u16>
}

#[derive(/*Clone, */Debug)]
pub struct Model {
    pub filename   : &'static str,
    pub model_mat  : Matrix4,
    pub world_mat  : Matrix4,
    pub mesh       : Mesh,
    pub texture_id : u16
}

//TODO replace Vecs with HeapArrays
#[derive(Debug)]
pub struct HeapArray<T> {
    pub data  : Box<[T]>,
    pub count : u64,
}


