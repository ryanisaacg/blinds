use crate::Event;
use winit::dpi::LogicalPosition;
use winit::event::{ElementState, MouseButton, WindowEvent};

// TODO: does this account for DPI?

#[derive(Clone, Debug, PartialEq)]
/// A simple mouse cursor abstraction
pub struct Mouse {
    position: LogicalPosition,
    left: bool,
    right: bool,
    middle: bool,
}

impl Mouse {
    /// Create a new mouse cache
    pub fn new() -> Mouse {
        Mouse {
            position: LogicalPosition {
                x: 0.0,
                y: 0.0
            },
            left: false,
            right: false,
            middle: false,
        }
    }

    /// Update the internal state to reflect the event
    pub fn process_event(&mut self, event: &Event) {
        match event {
            Event::Input(WindowEvent::CursorMoved { position, .. }) => self.position = *position,
            Event::Input(WindowEvent::MouseInput { state, button, .. }) => {
                let state = match state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                };
                match button {
                    MouseButton::Left => self.left = state,
                    MouseButton::Right => self.right = state,
                    MouseButton::Middle => self.middle = state,
                    _ => (),
                }
            }
            _ => ()
        }
    }

    pub fn position(&self) -> LogicalPosition {
        self.position
    }

    pub fn left(&self) -> bool {
        self.left
    }
    
    pub fn right(&self) -> bool {
        self.right
    }
    
    pub fn middle(&self) -> bool {
        self.middle
    }
}

