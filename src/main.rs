extern crate gl;
use gl::glutin as initgl;

fn main() {
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

    use gl::index::{NoIndices, PrimitiveType::TrianglesList};
    let indices = NoIndices(TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        uniform float t;
        void main() {
            vec2 pos = position;
            pos.x += t;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = gl::Program::from_source(
        &display, vertex_shader_src, fragment_shader_src, None)
        .expect("could not create shader program");

    let mut t : f32 = -0.5;
    let mut dt : f32 = std::time::Instant::now().elapsed().as_secs_f32();
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

        dt -= std::time::Instant::now().elapsed().as_secs_f32();
        t += dt;
        if t > 0.5 {
            t = -0.5;
        }

        use gl::Surface;
        let mut frame = display.draw();
        frame.clear_color(0.0, 1.0, 1.0, 1.0);

        let uniforms = gl::uniform! {
            matrix : [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0f32],
            ]
        };

        println!("t is {}", t);
        
        frame.draw(&vertex_buffer,
                   &indices,
                   &program,
                   &uniforms,
                   &Default::default()).unwrap();
        
        frame.finish().expect("could not swap buffers");
    });
}
