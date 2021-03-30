use crate::types::*;
use crate::model_parser::model::{Obj, Interleaved};

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices:     Vec<Vertex>,
    pub indices:      Vec<u16>,
    pub model_matrix: Matrix4,
    pub id:           u16,
}

impl Mesh {
    pub fn new(id: u16) -> Self {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            model_matrix: Matrix4::identity(),
            id
        }
    }
    pub fn load(name : &'static str,
                model_matrix: Matrix4,
                id: u16) -> Maybe<Self> {
        let obj_path = format!("data/models/{}.obj", name);
        //let mtl_path = format!("data/models/{}.mtl", name);
        
        let Interleaved{v_vt_vn, idx} = Obj::read_file(&obj_path)?
            .objects[0]
            .interleaved();

        //TODO this sucks do some slice shit 
        let vertices : Vec<Vertex> = v_vt_vn.iter()
            .map(|(v, vt, vn)|
                 Vertex {
                     position: [v.0, v.1, v.2, v.3],
                     uv: [vt.0, vt.1, vt.2],
                     normal: [vn.0, vn.1, vn.2],
                     color: [0.0, 0.0, 0.0, 0.0]
                 })
            .collect();


        
        let indices = idx.iter()
            .map(|x| *x as u16)
            .collect();

        
        /*
        let vertex_buffer = gl::VertexBuffer::new(display, &vertices)
            .expect("failed to create vertex buffer");
        
        let index_buffer = gl::IndexBuffer::new(
            display, gl::index::PrimitiveType::TrianglesList, &indices)
            .expect("failed to create index buffer");
gl::texture::srgb_texture2d::SrgbTexture2d
         */

        Ok(Mesh {vertices, indices, model_matrix, id})
    }
}
