use crate::types::*;
use crate::model_parser::model::{Obj, Interleaved};

impl Mesh {
    pub fn load(name : &'static str) -> Maybe<Self> {
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
                     normal: [vn.0, vn.1, vn.2]})
            .collect();

        
        let indices : Vec<u16> = idx.iter()
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

        Ok(Mesh { filename   : name,
                  texture_id : 0,
                  loaded     : false,
                  vertices, indices,
        })
    }
}
