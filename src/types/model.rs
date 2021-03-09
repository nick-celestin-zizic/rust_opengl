use crate::types::*;
use crate::model_parser::model::{Obj, Interleaved};

impl Model {
    pub fn load(name : &'static str, model : Matrix4) -> Maybe<Self> {
        let obj_path = format!("data/models/{}.obj", name);
        //let mtl_path = format!("data/models/{}.mtl", name);
        
        let Interleaved{v_vt_vn, idx} = Obj::read_file(&obj_path)?
            .objects[0]
            .interleaved();
        
        let vertices : Vec<Vertex> = v_vt_vn.iter()
            .map(|(v, vt, vn)| Vertex::from(*v, *vt, *vn))
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

        Ok(Model {
            filename   : name,
            model_mat  : model,
            world_mat  : Matrix4::identity(),
            mesh       : Mesh {vertices, indices},
            texture_id : 0,
        })
        
    }
}
