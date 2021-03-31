pub use glium::glutin as initgl;
pub use glium         as gl;
use crate::game::*;

pub use crate::types::*;

mod event_handler;
use event_handler::*;

use gl::{
    Surface, VertexBuffer, IndexBuffer,
    index::PrimitiveType::TrianglesList,
    draw_parameters::{
        BackfaceCullingMode::CullClockwise,
        DepthTest::IfLess,
    }
};

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


pub const WINDOW_WIDTH:  u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;

pub fn main() -> Maybe {
    let (display, event_loop) = {
        use initgl::{ event_loop::EventLoop,
                      window::WindowBuilder,
                      ContextBuilder,
                      dpi::PhysicalSize};
        
        let event_loop = EventLoop::new();
        let window     = WindowBuilder::new()
            .with_title("GABAGOL")
            .with_inner_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .with_resizable(false);
        let context = ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(true);
        let display = gl::Display::new(window, context, &event_loop)?;
        
        (display, event_loop)
    };

    let params = gl::DrawParameters {
        depth: gl::Depth {
            test:  IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: CullClockwise,
        .. Default::default()
    };
    
    //let funny = Model::load("box", Matrix4::identity())?;
    let basic_shader = compile_shader(&display, "basic")?;

    let projection = Matrix4::projection(
        60.0, WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32, 1.0, 100.0);

    //TODO more than one texture/palette
    let palette = {
        let file = std::fs::File::open("data/textures/palette.png")?;
        let file = std::io::BufReader::new(file);
        let file = image::load(file, image::ImageFormat::Png)?.to_rgba8();
        let dimensions = file.dimensions();
        let image = gl::texture::RawImage2d::from_raw_rgba_reversed(
            &file.into_raw(), dimensions);

        gl::texture::SrgbTexture2d::new(&display, image)?
    };

    //TODO better allocation
    use std::collections::HashMap;
    let mut vbos = HashMap::new();

    gl::implement_vertex!(Vertex, position, color, normal, uv);

    let mut meshes : HashMap<u16, Mesh> = HashMap::new();
    
    let mut input = Input::default();
    let mut state = GameState {
        camera: Matrix4::identity(),
        .. Default::default()
    };

    // controller stuff
    let api = hidapi::HidApi::new().unwrap();
    for device in api.device_list() {
        println!("vid: {:x}\t pid: {:x}",
                 device.vendor_id(), device.product_id());
    }
    //panic!();
    let controller_handle : Option<hidapi::HidDevice> = {
        if let Some(info) = api.device_list().next() {
            input.controller = Some(Controller::default());
            let handle = api.open(info.vendor_id(), info.product_id())
                .unwrap();
            handle.set_blocking_mode(true)?;
            Some(handle)
        } else { None }
    };
    
    event_loop.run(move |event, _, control_flow| {
        use initgl::event_loop::ControlFlow::Poll;
        use initgl::event::Event::MainEventsCleared;

        *control_flow = Poll;
        
        handle_event(&event, control_flow, &mut input);
        if let Some(handle) = &controller_handle {
            let mut controller = input.controller.as_mut().unwrap();
            let mut buff       = [0u8; 10];
            
            handle.read(&mut buff[..]).unwrap();
            
            controller.buttons = (buff[2] as u16) << 8 | buff[3] as u16;
            use crate::util::{normalize, deadzone};
            controller.left_stick  = [
                deadzone(
                    normalize(buff[6] as f32, 0.0, 255.0, -1.0, 1.0),
                    0.1),
                deadzone(
                    normalize(buff[7] as f32, 0.0, 255.0, -1.0, 1.0),
                    0.1),
            ];
            println!("{}\t{}", buff[8], buff[9]);
            controller.right_stick  = [
                deadzone(
                    normalize(buff[8] as f32, 0.0, 255.0, -1.0, 1.0),
                    0.1),
                deadzone(
                    normalize(buff[9] as f32, 0.0, 255.0, -1.0, 1.0),
                    0.1),
            ];
        }
        if let MainEventsCleared = event {
            game_update_and_render(&mut state, &mut meshes, &mut input);
            
            let mut frame = display.draw();
            frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
            
            for mesh in meshes.values() {
                let (vertices, indices) = &vbos.entry(mesh.id).or_insert((
                    VertexBuffer::new(&display, &mesh.vertices)
                        .unwrap_or_else(|e| panic!(
                            "could not load vertex buffer for mesh #{}:\n{}",
                            mesh.id, e)),
                    
                    IndexBuffer::new(&display, TrianglesList, &mesh.indices)
                        .unwrap_or_else(|e| panic!(
                            "could not load index buffer for mesh #{}:\n{}",
                            mesh.id, e))
                ));
                
                frame.draw(vertices, indices, &basic_shader,
                           &gl::uniform! { u_model:      mesh.model_matrix.0,
                                           u_view:       state.camera.0,
                                           u_projection: projection.0,
                                           u_texture:    &palette },
                           &params).unwrap_or_else(|e| panic!(
                               "could't not render mesh #{}\n{}",
                               mesh.id, e));
            }
            
            frame.finish().expect("COULD NOT SWAP BUFFERS");
        };
    });

}
