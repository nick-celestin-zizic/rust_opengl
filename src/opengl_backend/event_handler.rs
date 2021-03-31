use crate::opengl_backend::*;

use initgl::{event_loop::ControlFlow::*,
             event_loop::ControlFlow,
             event::*};

pub fn handle_event<T>(event:        &    Event<T>,
                       control_flow: &mut ControlFlow,
                       input:        &mut Input) {
    #[allow(clippy::collapsible_match)]
    #[allow(clippy::single_match)]
    match event {
        Event::WindowEvent {event, ..} => match event {
            WindowEvent::CloseRequested => {
                *control_flow = Exit;
            }
            WindowEvent::CursorMoved{position, ..} => {
                input.mouse[0] =
                    2.0 * ((position.x as f32)/(WINDOW_WIDTH as f32)) - 1.0;
                input.mouse[1] =
                    2.0 * ((position.y as f32)/(WINDOW_HEIGHT as f32)) - 1.0;
            }
            _ => ()
        }
        Event::DeviceEvent{event, ..} => match event {
            DeviceEvent::Key(KeyboardInput{scancode, state, ..}) => { match state {
                ElementState::Pressed  => match scancode {
                    1 => {
                        *control_flow = Exit
                    }
                    _ => ()
                }
                ElementState::Released => (),
            }},
            DeviceEvent::MouseWheel{delta} => {
                if let MouseScrollDelta::LineDelta(x, y) = delta{
                    input.scroll[0] += x;
                    input.scroll[1] += y;
                }
            },
            _ => ()
        }
        _ => ()
    }
}
