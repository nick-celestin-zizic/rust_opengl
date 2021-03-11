pub mod math;
pub use math::*;

pub mod matrix4;
pub use matrix4::*;

pub mod vertex;
pub use vertex::*;

pub mod model;
pub use model::*;

#[macro_use]
pub mod heap_array;
pub use heap_array::*;


pub type Maybe<T> = Result<T, Box<dyn std::error::Error>>;

#[allow(dead_code)]
pub type Vector2 = [f32; 2];
pub type Vector3 = [f32; 3];
pub type Vector4 = [f32; 4];

pub type Matrix4 = [[f32; 4]; 4];

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position   : Vector4,
    pub uv         : Vector3,
    pub normal     : Vector3,
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub filename   : &'static str,
    pub texture_id : u16,
    pub vertices   : Vec<Vertex>,
    pub indices    : Vec<u16>,
    pub loaded     : bool,
}

//TODO replace Vecs with HeapArrays
#[derive(Debug, Clone)]
pub struct HeapArray<T> {
    pub data  : Box<[T]>,
    pub count : u64,
}

