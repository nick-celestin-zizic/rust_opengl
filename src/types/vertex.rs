use crate::types::*;

type GlTuple4 = (f32, f32, f32, f32);
type GlTuple3 = (f32, f32, f32);
impl Vertex {
    pub fn from(v : GlTuple4, vt : GlTuple3, vn : GlTuple3) -> Vertex {
        //TODO do mtl stuff and hook up texture ids
        Vertex { position   : [ v.0,  v.1,  v.2, v.3],
                 normal     : [vn.0, vn.1, vn.2],
                 uvw        : [vt.0, vt.1, vt.2] }
    }
}
