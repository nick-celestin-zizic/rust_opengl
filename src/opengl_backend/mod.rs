use crate::glium::glutin as initgl;
use crate::glium         as gl;

use crate::types::*;

use gl::{Display, gl::program::Program};

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
    gl::implement_vertex!(Vertex, position, uvw, normal);

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
    
    let funny = Model::load("box", Matrix4::identity())?;
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
    use gl::{VertexBuffer, IndexBuffer, index::PrimitiveType::TrianglesList};
    let vertex_buffer = VertexBuffer::new(&display, &funny.mesh.vertices)?;
    let index_buffer  = IndexBuffer::new(&display,
                                         TrianglesList,
                                         &funny.mesh.indices)?;
    let texture = {
        let file = std::fs::File::open("data/textures/palette.png")?;
        let file = std::io::BufReader::new(file);
        let file = image::load(file, image::ImageFormat::Png)?.to_rgba8();
        let dimensions = file.dimensions();
        let image = gl::texture::RawImage2d::from_raw_rgba_reversed(&file.into_raw(), dimensions);

        gl::texture::SrgbTexture2d::new(&display, image)?
    };
    

    //let mut t : f32 = 0.01;
    event_loop.run(move |event, _, control_flow| {
        use initgl::event_loop::ControlFlow::*;
        use initgl::event::*;

        *control_flow = Poll;

        #[allow(clippy::collapsible_match)]
        #[allow(clippy::single_match)]
        match event {
            Event::WindowEvent {event, ..} => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = Exit;
                }
                _ => ()
            }
            Event::DeviceEvent{event, ..} => match event {
                DeviceEvent::Key(KeyboardInput{scancode, state, ..}) => {
                    match state {
                        ElementState::Pressed  => match scancode {
                            1 => {
                                *control_flow = Exit
                            }
                            _ => ()
                        }
                        
                        ElementState::Released => ()
                    }
                }
                _ => ()
            }
            _ => ()
        }
        
        //t += 0.01;
        
        use gl::Surface;
        let mut frame = display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let params = gl::DrawParameters {
            depth: gl::Depth {
                test: gl::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            //backface_culling : gl::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

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

        frame.draw(&vertex_buffer,
                   &index_buffer,
                   &basic_shader,
                   &gl::uniform! { u_model       : model,
                                   u_view        : view,
                                   u_perspective : perspective,
                                   u_light       : light,
                                   u_texture     : &texture },
                   &params).expect("triangle fucked up");
        
        frame.finish().expect("could not swap buffers");
    });
}
