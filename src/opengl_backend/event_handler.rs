use crate::opengl_backend::*;

use initgl::{event_loop::ControlFlow::*,
             event_loop::ControlFlow,
             event::*};

pub fn handle_event<T>(event:             Event<T>,
                       control_flow: &mut ControlFlow,
                       projection:   &mut Matrix4) {
    #[allow(clippy::collapsible_match)]
    #[allow(clippy::single_match)]
    match event {
        Event::WindowEvent {event, ..} => match event {
            WindowEvent::CloseRequested => {
                *control_flow = Exit;
            }
            WindowEvent::Resized(size) => {
                *projection =
                    Matrix4::projection(60.0,
                                        size.width as f32/size.height as f32,
                                        1.0, 100.0);
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
}
