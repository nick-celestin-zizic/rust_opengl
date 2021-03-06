extern crate model_parser;
extern crate gl;

use gl::glutin as initgl;

mod types;
use types::{Vertex, Matrix4};


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
    let (display, event_loop) = {
        use initgl::{ event_loop::EventLoop,
                      window::WindowBuilder,
                      dpi::LogicalSize,
                      ContextBuilder,       };

        let event_loop = EventLoop::new();
        let window     = WindowBuilder::new()
            .with_title("GABAGOL")
            .with_inner_size(LogicalSize::new(400, 600));
        let context = ContextBuilder::new();
        let display = gl::Display::new(window, context, &event_loop)
            .expect("failed to create display");

        (display, event_loop)
    };

    gl::implement_vertex!(Vertex, position, normal, texture);

    let _params = gl::DrawParameters {
        depth : gl::Depth {
            test  : gl::draw_parameters::DepthTest::IfLess,
            write : true,
            .. Default::default()
        },
        .. Default::default()
    };


    let (vertex_buffer, index_buffer) = load_model(&display, "teapot");
    let triangle_shader = compile_shader(&display, "basic");
    let projection : Matrix4 = [
        [ 1.0,  0.0,  0.0,  0.0],
        [ 0.0,  1.0,  0.0,  0.0],
        [ 0.0,  0.0,  1.0, 0.0],
        [ 0.0,  0.0,  0.0,  1.0]
    ];
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
        
        

        //t += 0.001;

        
        use gl::Surface;
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        
        frame.draw(&vertex_buffer,
                   &index_buffer,
                   &triangle_shader,
                   &gl::uniform! {matrix : projection},
                   &Default::default()).expect("triangle fucked up");
        
        frame.finish().expect("could not swap buffers");
    });
}
