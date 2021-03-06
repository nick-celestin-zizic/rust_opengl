extern crate model_parser;
extern crate gl;

use gl::glutin as initgl;

mod types;
use types::*;

fn compile_shader (display : &gl::Display,
                   name : &str) -> gl::program::Program {
    let vertex_name   = format!("src/shaders/vert_{}.glsl", name);
    let fragment_name = format!("src/shaders/frag_{}.glsl", name);

    #[inline] fn get_src (name : &String) -> String {
        std::fs::read_to_string(std::path::Path::new(name))
            .expect(&format!("could not load shader '{}'", name))
    }

    let vertex_src   = get_src(&vertex_name);
    let fragment_src = get_src(&fragment_name);

    gl::Program::from_source(display, &vertex_src, &fragment_src, None)
        .expect(&format!("could not compile shader '{}'", name))
}

fn load_model (display : &gl::Display,
               name : &str) -> (gl::VertexBuffer<Vertex>,
                                gl::IndexBuffer<u16>) {
    use model_parser::model::{Obj, Interleaved};

    let obj_path = format!("data/models/{}.obj", name);
    //let mtl_path = format!("data/models/{}.mtl", name);
    
    let Interleaved{v_vt_vn, idx} = Obj::read_file(&obj_path)
        .expect(&format!("could not load model '{}'", name))
        .objects[0]
        .interleaved();
    
    let vertices : Vec<Vertex> = v_vt_vn.iter()
        .map(|v| Vertex::from(*v))
        .collect();

    let indices : Vec<u16> = idx.iter()
        .map(|x| *x as u16)
        .collect();

    let vertex_buffer = gl::VertexBuffer::new(display, &vertices)
        .expect("failed to create vertex buffer");
    
    let index_buffer = gl::IndexBuffer::new(
        display, gl::index::PrimitiveType::TrianglesList, &indices)
        .expect("failed to create index buffer");

    (vertex_buffer, index_buffer)
}

fn main () {
    gl::implement_vertex!(Vertex, position, normal, texture);

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
        let display = gl::Display::new(window, context, &event_loop)
            .expect("failed to create display");
        
        (display, event_loop)
    };


    let (vertex_buffer, index_buffer) = load_model(&display, "teapot");
    let basic_shader = compile_shader(&display, "basic");

    let color : Vector3 = [1.0, 0.0, 0.0];
    let light : Vector3 = [-1.0, 0.4, 0.9];
    let model : Matrix4 = [
        [ 3.0,  0.0,  0.0,  0.0],
        [ 0.0,  3.0,  0.0,  0.0],
        [ 0.0,  0.0,  1.0,  0.0],
        [ 0.0,  0.0,  2.0,  1.0]
    ];
    let view = Matrix4::view([ 2.0, -1.0,  1.0],
                             [-2.0,  1.0,  1.0],
                             [ 0.0,  1.0,  0.0]);

    //let mut t : f32 = 0.01;
    event_loop.run(move |event, _, control_flow| {
        use initgl::event_loop::ControlFlow::*;
        use initgl::event::*;

        *control_flow = Poll;
        
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

            let fov : f32 = 3.141592 / 3.0;
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
                   &gl::uniform! { u_color       : color,
                                   u_light       : light,
                                   u_model       : model,
                                   u_view        : view,
                                   u_perspective : perspective },
                   &params).expect("triangle fucked up");
        
        frame.finish().expect("could not swap buffers");
    });
}
