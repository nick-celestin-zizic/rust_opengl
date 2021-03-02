extern crate gl;
use gl::glutin as initgl;

use gl::program::Program;
fn compile_shader (display : &gl::Display, name : &str) -> Program {
    let vertex_name   = String::from("src/shaders/vert_") + name + ".glsl";
    let fragment_name = String::from("src/shaders/frag_") + name + ".glsl";

    fn get_src (name : &String) -> String {
        let path = std::path::Path::new(name);
        return std::fs::read_to_string(path)
            .expect(&format!("could not load shader '{}'", name)[..]);
    }

    let vertex_src   = get_src(&vertex_name);
    let fragment_src = get_src(&fragment_name);

    return gl::Program::from_source(display, &vertex_src[..], &fragment_src[..], None)
        .expect(&format!("could not compile shader '{}'", name)[..]);
}

fn main () {
    let (display, event_loop) = {
        use initgl::*;

        let event_loop = event_loop::EventLoop::new();
        let window = window::WindowBuilder::new()
            .with_title("GABAGOL")
            .with_inner_size(dpi::LogicalSize::new(400, 600));
        let context = ContextBuilder::new();
        let display = gl::Display::new(window, context, &event_loop)
            .expect("failed to crate display");

        (display, event_loop)
    };

    #[derive(Copy, Clone, Debug)]
    struct Vertex {
        position : [f32; 2]
    }
    gl::implement_vertex!(Vertex, position);

    let shape = [
        Vertex {position : [-0.5, -0.5]},
        Vertex {position : [ 0.0,  0.5]},
        Vertex {position : [ 0.5, -0.25]}
    ];

    let vertex_buffer = gl::VertexBuffer::new(&display, &shape)
        .expect("failed to create vertex buffer");

    let program = compile_shader(&display, "triangle");
    
    let mut t : f32 = -0.5;
    //let mut dt : f32 = std::time::Instant::now().elapsed().as_secs_f32();
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
            _ => ()
        }

        //dt -= std::time::Instant::now().elapsed().as_secs_f32();
        t += 0.0002;

        use gl::Surface;
        let mut frame = display.draw();
        frame.clear_color(0.0, 1.0, 1.0, 1.0);

        let matrix = [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0,      0.0,     1.0, 0.0],
                [0.0,      0.0,     0.0, 1.0f32],
        ];

        let perspective = {
            let (width, height) = frame.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov   : f32 = 3.141592 / 3.0;
            let zfar  : f32 = 1024.0;
            let znear : f32 = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [[f * aspect_ratio, 0.0, 0.0                           , 0.0],
             [0.0             , f  , 0.0                           , 0.0],
             [0.0             , 0.0, (zfar+znear)/(zfar-znear)     , 1.0],
             [0.0             , 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0]]
        };

        let light = [-1.0, 0.4, 0.9f32];

        let _params = gl::DrawParameters {
            depth : gl::Depth {
                test  : gl::draw_parameters::DepthTest::IfLess,
                write : true,
                .. Default::default()
            },
            .. Default::default()
        };

        use gl::index::*;
        frame.draw(&vertex_buffer,
                   &NoIndices(PrimitiveType::TrianglesList),
                   &program,
                   &gl::uniform! { matrix : matrix,
                                   perspective : perspective,
                                   u_light : light
                   },
                   &Default::default()).expect("triangle fucked up");
        
        frame.finish().expect("could not swap buffers");
    });
}
