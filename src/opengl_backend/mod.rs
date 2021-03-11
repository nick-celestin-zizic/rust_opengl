pub extern crate glium;
pub use glium::glutin as initgl;
pub use glium         as gl;

use crate::game::game_update_and_render;

pub use crate::types::*;

mod event_handler;
use event_handler::*;

use gl::{Display, program::Program};

fn compile_shader (display : &Display, name : &str) -> Maybe<Program> {
    use regex::Regex;
    use std::{fs::read_to_string, path::Path};
    
    let text = read_to_string(Path::new(&format!("src/shaders/{}.glsl", name)))?;
    let (vertex_src, fragment_src) = {
        let srcs : Vec<&str> = Regex::new("(^|\\n)#shader_\\w+ *\\n")?
            .split(&text)
            .collect();
        
        (srcs[1], srcs[2])
    };

    let shader = gl::Program::from_source(display, &vertex_src, &fragment_src, None)?;

    Ok(shader)
}


pub fn main() -> Maybe<()> {
    let (display, event_loop) = {
        use initgl::{ event_loop::EventLoop,
                      window::WindowBuilder,
                      dpi::LogicalSize,
                      ContextBuilder,       };

        let event_loop = EventLoop::new();
        let window     = WindowBuilder::new()
            .with_title("GABAGOL")
            .with_inner_size(LogicalSize::new(400, 600));
        let context = ContextBuilder::new()
            .with_depth_buffer(24);
        let display = gl::Display::new(window, context, &event_loop)?;
        
        (display, event_loop)
    };

    let params = gl::DrawParameters {
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
    
    let light : Vector3 = [-1.0, 0.4, 0.9];
    let model : Matrix4 = [
        [ 0.1,  0.0,  0.0,  0.0],
        [ 0.0,  0.1,  0.0,  0.0],
        [ 0.0,  0.0,  1.0,  0.0],
        [ 0.0,  0.0,  0.0,  1.0]
    ];

    let view = Matrix4::view([ 2.0, -1.0,  1.0],
                             [-2.0,  1.0,  1.0],
                             [ 0.0,  1.0,  0.0]);


    // TODO change this to be smarter
    /*
    
     */
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
    let mut meshes : Vec<Mesh> = Vec::with_capacity(20);
    let mut vbos = HashMap::new();

    gl::implement_vertex!(Vertex, position, uv, normal);

    event_loop.run(move |event, _, control_flow| {
        use initgl::event_loop::ControlFlow::Poll;

        *control_flow = Poll;

        handle_event(event, control_flow);

        game_update_and_render(&mut meshes);
        
        use gl::Surface;
        let mut frame = display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        //TODO dont do this every frame
        let perspective = {
            let (width, height) = frame.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov : f32 = std::f32::consts::PI / 3.0;
            let zfar  = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [[f *   aspect_ratio, 0.0,              0.0              , 0.0],
             [         0.0      ,  f ,              0.0              , 0.0],
             [         0.0      , 0.0,  (zfar+znear)/(zfar-znear)    , 1.0],
             [         0.0      , 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0]]
        };


        for mesh in &meshes {
            #[allow(clippy::map_entry)]
            if !vbos.contains_key(&mesh.filename) {
                use gl::{VertexBuffer, IndexBuffer,
                         index::PrimitiveType::TrianglesList};
                
                let vertex_buffer = VertexBuffer::new(&display,
                                                      &mesh.vertices)
                    .unwrap_or_else(|_| panic!(
                        "could not load vertex buffer for mesh '{}'",
                        mesh.filename));
                
                let index_buffer  = IndexBuffer::new(&display,
                                                     TrianglesList,
                                                     &mesh.indices).unwrap();
                vbos.insert(mesh.filename, (vertex_buffer, index_buffer));
            }

            let (vertices, indices) = vbos.get(mesh.filename).unwrap();
            frame.draw(vertices, indices, &basic_shader,
                       &gl::uniform! { u_model       : model,
                                       u_view        : view,
                                       u_perspective : perspective,
                                       u_light       : light,
                                       u_texture     : &palette },
                       &params)
                .unwrap_or_else(|_| panic!("could't not render mesh "));
        }

        
        
        frame.finish().expect("COULD NOT SWAP BUFFERS");
    });
}
