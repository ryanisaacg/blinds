use crate::Event;
use std::fmt::{Debug, Error, Formatter};
use winit::event::{ElementState, WindowEvent, VirtualKeyCode};

#[derive(Copy)]
/// A simple struct to keep track of what keys are held
pub struct Keyboard {
    keys: [bool; 256],
}

impl Keyboard {
    /// Create a new keyboard cache
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [false; 256]
        }
    }

    /// Update the internal state to reflect the event
    pub fn process_event(&mut self, event: &Event) {
        if let Event::Input(WindowEvent::KeyboardInput { input, .. }) = event {
            if let Some(keycode) = input.virtual_keycode {
                let state = match input.state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                };
                let key = keycode as usize;
                self.keys[key] = state;
            }
        }
    }

    /// Check if a button is being held down
    pub fn is_down(&self, keycode: VirtualKeyCode) -> bool {
        self.keys[keycode  as usize]
    }
}

impl Clone for Keyboard {
    fn clone(&self) -> Keyboard {
        *self
    }
}

impl Debug for Keyboard {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Keyboard {{ keys: [")?;
        for key in self.keys.iter() {
            write!(f, "{:?}, ", key)?;
        }
        write!(f, "] }}")
    }
}

