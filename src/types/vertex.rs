use crate::types::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex {
    pub position: Vector4,
    pub color:    Vector4,
    pub normal:   Vector3,
    pub uv:       Vector3,
}

