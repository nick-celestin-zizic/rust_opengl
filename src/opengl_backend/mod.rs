pub use glium::glutin as initgl;
pub use glium         as gl;

use crate::game::*;

pub use crate::types::*;

mod event_handler;
use event_handler::*;

fn compile_shader (display: &gl::Display, name: &str) -> Maybe<gl::Program> {
    use regex::Regex;
    
    let text =std::fs::read_to_string(&format!("src/shaders/{}.glsl", name))?;
    
    let (vertex_src, fragment_src) = {
        let srcs : Vec<&str> = Regex::new("(^|\\n)#shader_\\w+ *\\n")?
            .split(&text)
            .collect();
        
        (srcs[1], srcs[2])
    };

    let shader = gl::Program::from_source(display, &vertex_src,
                                          &fragment_src, None)?;
    
    Ok(shader)
}


pub fn main() -> Maybe {
    let (display, event_loop) = {
        use initgl::{ event_loop::EventLoop,
                      window::WindowBuilder,
                      ContextBuilder,       };
        
        let event_loop = EventLoop::new();
        let window     = WindowBuilder::new()
            .with_title("GABAGOL");
        let context = ContextBuilder::new()
            .with_depth_buffer(24);
        let display = gl::Display::new(window, context, &event_loop)?;
        
        (display, event_loop)
    };

    let mut params = gl::DrawParameters {
        depth: gl::Depth {
            test: gl::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling : gl::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };
    
    //let funny = Model::load("box", Matrix4::identity())?;
    let basic_shader = compile_shader(&display, "basic")?;

    let view = {
        let mut mat = Matrix4::identity();
        mat.translate(0.0, 0.0, -5.0);
        mat
    };
    let mut projection = Matrix4::identity();


    //TODO more than one texture/palette
    let palette = {
        let file = std::fs::File::open("data/textures/palette.png")?;
        let file = std::io::BufReader::new(file);
        let file = image::load(file, image::ImageFormat::Png)?.to_rgba8();
        let dimensions = file.dimensions();
        let image = gl::texture::RawImage2d::from_raw_rgba_reversed(&file.into_raw(), dimensions);

        gl::texture::SrgbTexture2d::new(&display, image)?
    };

    //TODO better allocation
    use std::collections::HashMap;
    let mut vbos = HashMap::new();

    gl::implement_vertex!(Vertex, position, uv);//, normal);

    let mut meshes : HashMap<u16, Mesh> = HashMap::new();
    let mut state = GameState::init();
    
    event_loop.run(move |event, _, control_flow| {
        use initgl::event_loop::ControlFlow::Poll;

        *control_flow = Poll;

        handle_event(event, control_flow, &mut projection);

        game_update_and_render(&mut state, &mut meshes);
        
        use gl::Surface;
        let mut frame = display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let (width, height) = frame.get_dimensions();
        params.viewport = Some(gl::Rect {left:0,bottom:0, width, height});

        for mesh in meshes.values() {
            #[allow(clippy::map_entry)]
            if !vbos.contains_key(&mesh.id) {
                use gl::{VertexBuffer, IndexBuffer,
                         index::PrimitiveType::TrianglesList};
                
                let vertex_buffer = VertexBuffer::new(&display,&mesh.vertices)
                    .unwrap_or_else(|e| panic!(
                        "could not load vertex buffer for mesh #{}:\n{}",
                        mesh.id, e));
                
                let index_buffer =
                    IndexBuffer::new(&display, TrianglesList, &mesh.indices)
                    .unwrap_or_else(|e| panic!(
                        "could not load index buffer for mesh #{}:\n{}",
                        mesh.id, e));
                
                vbos.insert(mesh.id, (vertex_buffer, index_buffer));
            }

            let (vertices, indices) = vbos.get(&mesh.id).unwrap();
            frame.draw(vertices, indices, &basic_shader,
                       &gl::uniform! { u_model:      mesh.model_matrix.0,
                                       u_view:       view.0,
                                       u_projection: projection.0,
                                       u_texture:    &palette },
                       &params).unwrap_or_else(
                |e| panic!("could't not render mesh #{}\n{}", mesh.id, e));
        }
        
        frame.finish().expect("COULD NOT SWAP BUFFERS");
    });
}
