use mint::Vector2;
mod convert;
mod gamepad;
mod keyboard;
mod pointer;

pub(crate) use self::convert::*;
pub use self::gamepad::*;
pub use self::keyboard::*;
pub use self::pointer::*;

#[derive(Clone, Debug)]
#[non_exhaustive]
/// An indicator something has changed or input has been dispatched
pub enum Event {
    /// The size of the window has changed, see [`Window::size`]
    ///
    /// [`Window::size`]: crate::Window::size
    Resized(ResizedEvent),
    /// The scale factor of the window has changed, see [`Window::scale_factor`]
    ///
    /// [`Window::scale_factor`]: crate::Window::scale_factor
    ScaleFactorChanged(ScaleFactorChangedEvent),
    /// The window has gained operating system focus (true), or lost it (false)
    FocusChanged(FocusChangedEvent),
    /// The user typed a character, used for text input
    ///
    /// Don't use keyboard events for text! Depending on how the user's operating system and
    /// keyboard layout are configured, different keys may produce different Unicode characters.
    ReceivedCharacter(ReceivedCharacterEvent),
    /// A key has been pressed, released, or held down
    ///
    /// Operating systems often have key repeat settings that cause duplicate events to be
    /// generated for a single press.
    KeyboardInput(KeyboardEvent),
    /// The mouse entered the window
    PointerEntered(PointerEnteredEvent),
    /// The mouse left the window
    PointerLeft(PointerLeftEvent),
    /// The mouse pointer has a new position within the window
    PointerMoved(PointerMovedEvent),
    /// A mouse button has been pressed or released
    PointerInput(PointerInputEvent),
    /// The mousewheel has scrolled, either in lines or pixels (depending on the input method)
    ScrollInput(ScrollDelta),
    /// The keyboard modifiers have changed.
    ModifiersChanged(ModifiersChangedEvent),
    /// A gamepad has been connected
    GamepadConnected(GamepadConnectedEvent),
    /// A gamepad has been connected
    GamepadDisconnected(GamepadDisconnectedEvent),
    /// A gamepad has been connected
    GamepadButton(GamepadButtonEvent),
    /// A gamepad has been connected
    GamepadAxis(GamepadAxisEvent),
}


#[derive(Clone, Debug)]
pub struct ResizedEvent {
    pub(crate) size: Vector2<f32>
}

impl ResizedEvent {
    pub fn logical_size(&self) -> Vector2<f32> {
        self.size
    }
}

#[derive(Clone, Debug)]
pub struct ScaleFactorChangedEvent {
    pub(crate) scale: f32
}

impl ScaleFactorChangedEvent {
    pub fn scale_factor(&self) -> f32 {
        self.scale
    }
}

#[derive(Clone, Debug)]
pub struct FocusChangedEvent {
    pub(crate) focus: bool
}

impl FocusChangedEvent {
    pub fn is_focused(&self) -> bool {
        self.focus
    }
}

#[derive(Clone, Debug)]
pub struct ReceivedCharacterEvent {
    pub(crate) chr: char
}

impl ReceivedCharacterEvent {
    pub fn character(&self) -> char {
        self.chr
    }
}


#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
/// A change in the event modifiers like shift, control, alt, or 'logo'
pub struct ModifiersChangedEvent {
    shift: bool,
    ctrl: bool,
    alt: bool,
    logo: bool,
}

impl ModifiersChangedEvent {
    pub fn shift(&self) -> bool {
        self.shift
    }

    pub fn ctrl(&self) -> bool {
        self.ctrl
    }

    pub fn alt(&self) -> bool {
        self.alt
    }

    /// Windows, Command, etc.
    pub fn logo(&self) -> bool {
        self.logo
    }
}
