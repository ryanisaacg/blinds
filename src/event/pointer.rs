use mint::Vector2;
use winit::dpi::LogicalPosition;
use winit::event::{DeviceId, MouseScrollDelta as MSD};

#[derive(Clone, Debug)]
pub struct PointerEnteredEvent(pub(crate) Pointer);

impl PointerEnteredEvent {
    pub fn pointer(&self) -> Pointer {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct PointerLeftEvent(pub(crate) Pointer);

impl PointerLeftEvent {
    pub fn pointer(&self) -> Pointer {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct PointerMovedEvent {
    pub(crate) id: Pointer,
    pub(crate) location: Vector2<f32>,
}

impl PointerMovedEvent {
    pub fn pointer(&self) -> Pointer {
        self.id
    }

    pub fn location(&self) -> Vector2<f32> {
        self.location
    }
}

#[derive(Clone, Debug)]
pub struct PointerInputEvent {
    pub(crate) id: Pointer,
    pub(crate) button: MouseButton,
    pub(crate) is_down: bool,
}

impl PointerInputEvent {
    pub fn pointer(&self) -> Pointer {
        self.id
    }

    pub fn button(&self) -> MouseButton {
        self.button
    }

    pub fn is_down(&self) -> bool {
        self.is_down
    }
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Debug, Hash)]
/// A unique ID for multiple mouse pointers
pub struct Pointer(pub(crate) DeviceId);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
/// A button on a standard 3-button mouse
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u8),
}

impl From<winit::event::MouseButton> for MouseButton {
    fn from(mb: winit::event::MouseButton) -> Self {
        match mb {
            winit::event::MouseButton::Left => MouseButton::Left,
            winit::event::MouseButton::Right => MouseButton::Right,
            winit::event::MouseButton::Middle => MouseButton::Middle,
            winit::event::MouseButton::Other(x) => MouseButton::Other(x),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// A measure of how much was scrolled in an event
pub enum ScrollDelta {
    /// This many lines of text were scrolled
    Lines(Vector2<f32>),
    /// This many input pixels were scrolled
    Pixels(Vector2<f32>),
}

impl From<MSD> for ScrollDelta {
    fn from(msd: MSD) -> Self {
        match msd {
            MSD::LineDelta(x, y) => Self::Lines(Vector2 { x, y }),
            MSD::PixelDelta(LogicalPosition { x, y }) => Self::Pixels(Vector2 {
                x: x as f32,
                y: y as f32,
            }),
        }
    }
}
