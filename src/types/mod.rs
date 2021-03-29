pub mod math;
pub use math::*;

pub mod matrix4;
pub use matrix4::*;

pub mod vertex;
pub use vertex::*;

pub mod mesh;
pub use mesh::*;

#[macro_use]
pub mod heap_array;
pub use heap_array::*;

pub mod bucket_array;
pub use bucket_array::*;

pub type Maybe<T = ()> = Result<T, Box<dyn std::error::Error>>;
pub type Meshes = std::collections::HashMap<u16, Mesh>;

#[allow(dead_code)]
pub type Vector2 = [f32; 2];
pub type Vector3 = [f32; 3];
pub type Vector4 = [f32; 4];









