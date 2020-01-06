use mint::Vector2;
use std::cmp::Ordering;
use winit::dpi::LogicalPosition;
use winit::event::{DeviceId, MouseScrollDelta as MSD, VirtualKeyCode};

#[derive(Debug)]
/// An indicator something has changed or input has been dispatched
pub enum Event {
    /// The size of the window has changed, see [`Window::size`]
    ///
    /// [`Window::size`]: crate::Window::size
    Resized(Vector2<f32>),
    /// The scale factor of the window has changed, see [`Window::scale_factor`]
    ///
    /// [`Window::scale_factor`]: crate::Window::scale_factor
    ScaleFactorChanged(f32),
    /// The window has gained operating system focus (true), or lost it (false)
    Focused(bool),
    /// The user typed a character, used for text input
    ///
    /// Don't use keyboard events for text! Depending on how the user's operating system and
    /// keyboard layout are configured, different keys may produce different Unicode characters.
    ReceivedCharacter(char),
    /// A key has been pressed, released, or held down
    ///
    /// Operating systems often have key repeat settings that cause duplicate events to be
    /// generated for a single press.
    KeyboardInput { key: Key, state: ElementState },
    /// A given pointer has entered the window
    MouseEntered { pointer: Pointer },
    /// A given pointer has left the window
    MouseLeft { pointer: Pointer },
    /// The pointer has a new position within the window
    MouseMoved {
        pointer: Pointer,
        position: Vector2<f32>,
    },
    /// The mousewheel has scrolled, either in lines or pixels (depending on the input method)
    MouseWheel {
        pointer: Pointer,
        delta: MouseScrollDelta,
    },
    /// A mouse button has been pressed or released
    MouseInput {
        pointer: Pointer,
        state: ElementState,
        button: MouseButton,
    },

    /// The keyboard modifiers have changed.
    ModifiersChanged { modifiers: Modifiers },

    /// A gamepad button has been pressed or released, or an axis has changed
    GamepadEvent { id: GamepadId, event: GamepadEvent },
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
/// The active keyboard modifiers (shift, ctrl, etc.) when an event was dispatched
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    /// Windows, Command, etc.
    pub logo: bool,
}

impl From<winit::event::ModifiersState> for Modifiers {
    fn from(modifiers: winit::event::ModifiersState) -> Modifiers {
        Modifiers {
            shift: modifiers.shift(),
            ctrl: modifiers.ctrl(),
            alt: modifiers.alt(),
            logo: modifiers.logo(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
/// A button or key's state, either Pressed or Released
pub enum ElementState {
    Pressed,
    Released,
}

impl From<winit::event::ElementState> for ElementState {
    fn from(modifier: winit::event::ElementState) -> ElementState {
        use winit::event::ElementState::*;

        match modifier {
            Pressed => ElementState::Pressed,
            Released => ElementState::Released,
        }
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
pub enum MouseScrollDelta {
    /// This many lines of text were scrolled
    Lines(Vector2<f32>),
    /// This many input pixels were scrolled
    Pixels(Vector2<f32>),
}

impl From<MSD> for MouseScrollDelta {
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

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
/// A key location on a keyboard
pub enum Key {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Snapshot,
    Scroll,
    Pause,
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,
    Left,
    Up,
    Right,
    Down,
    Back,
    Return,
    Space,
    Compose,
    Caret,
    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    AbntC1,
    AbntC2,
    Add,
    Apostrophe,
    Apps,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Decimal,
    Divide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Multiply,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Subtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Cut,
    Copy,
    Paste,
}

impl From<VirtualKeyCode> for Key {
    fn from(key: VirtualKeyCode) -> Key {
        match key {
            VirtualKeyCode::Key1 => Key::Key1,
            VirtualKeyCode::Key2 => Key::Key2,
            VirtualKeyCode::Key3 => Key::Key3,
            VirtualKeyCode::Key4 => Key::Key4,
            VirtualKeyCode::Key5 => Key::Key5,
            VirtualKeyCode::Key6 => Key::Key6,
            VirtualKeyCode::Key7 => Key::Key7,
            VirtualKeyCode::Key8 => Key::Key8,
            VirtualKeyCode::Key9 => Key::Key9,
            VirtualKeyCode::Key0 => Key::Key0,
            VirtualKeyCode::A => Key::A,
            VirtualKeyCode::B => Key::B,
            VirtualKeyCode::C => Key::C,
            VirtualKeyCode::D => Key::D,
            VirtualKeyCode::E => Key::E,
            VirtualKeyCode::F => Key::F,
            VirtualKeyCode::G => Key::G,
            VirtualKeyCode::H => Key::H,
            VirtualKeyCode::I => Key::I,
            VirtualKeyCode::J => Key::J,
            VirtualKeyCode::K => Key::K,
            VirtualKeyCode::L => Key::L,
            VirtualKeyCode::M => Key::M,
            VirtualKeyCode::N => Key::N,
            VirtualKeyCode::O => Key::O,
            VirtualKeyCode::P => Key::P,
            VirtualKeyCode::Q => Key::Q,
            VirtualKeyCode::R => Key::R,
            VirtualKeyCode::S => Key::S,
            VirtualKeyCode::T => Key::T,
            VirtualKeyCode::U => Key::U,
            VirtualKeyCode::V => Key::V,
            VirtualKeyCode::W => Key::W,
            VirtualKeyCode::X => Key::X,
            VirtualKeyCode::Y => Key::Y,
            VirtualKeyCode::Z => Key::Z,
            VirtualKeyCode::Escape => Key::Escape,
            VirtualKeyCode::F1 => Key::F1,
            VirtualKeyCode::F2 => Key::F2,
            VirtualKeyCode::F3 => Key::F3,
            VirtualKeyCode::F4 => Key::F4,
            VirtualKeyCode::F5 => Key::F5,
            VirtualKeyCode::F6 => Key::F6,
            VirtualKeyCode::F7 => Key::F7,
            VirtualKeyCode::F8 => Key::F8,
            VirtualKeyCode::F9 => Key::F9,
            VirtualKeyCode::F10 => Key::F10,
            VirtualKeyCode::F11 => Key::F11,
            VirtualKeyCode::F12 => Key::F12,
            VirtualKeyCode::F13 => Key::F13,
            VirtualKeyCode::F14 => Key::F14,
            VirtualKeyCode::F15 => Key::F15,
            VirtualKeyCode::F16 => Key::F16,
            VirtualKeyCode::F17 => Key::F17,
            VirtualKeyCode::F18 => Key::F18,
            VirtualKeyCode::F19 => Key::F19,
            VirtualKeyCode::F20 => Key::F20,
            VirtualKeyCode::F21 => Key::F21,
            VirtualKeyCode::F22 => Key::F22,
            VirtualKeyCode::F23 => Key::F23,
            VirtualKeyCode::F24 => Key::F24,
            VirtualKeyCode::Snapshot => Key::Snapshot,
            VirtualKeyCode::Scroll => Key::Scroll,
            VirtualKeyCode::Pause => Key::Pause,
            VirtualKeyCode::Insert => Key::Insert,
            VirtualKeyCode::Home => Key::Home,
            VirtualKeyCode::Delete => Key::Delete,
            VirtualKeyCode::End => Key::End,
            VirtualKeyCode::PageDown => Key::PageDown,
            VirtualKeyCode::PageUp => Key::PageUp,
            VirtualKeyCode::Left => Key::Left,
            VirtualKeyCode::Up => Key::Up,
            VirtualKeyCode::Right => Key::Right,
            VirtualKeyCode::Down => Key::Down,
            VirtualKeyCode::Back => Key::Back,
            VirtualKeyCode::Return => Key::Return,
            VirtualKeyCode::Space => Key::Space,
            VirtualKeyCode::Compose => Key::Compose,
            VirtualKeyCode::Caret => Key::Caret,
            VirtualKeyCode::Numlock => Key::Numlock,
            VirtualKeyCode::Numpad0 => Key::Numpad0,
            VirtualKeyCode::Numpad1 => Key::Numpad1,
            VirtualKeyCode::Numpad2 => Key::Numpad2,
            VirtualKeyCode::Numpad3 => Key::Numpad3,
            VirtualKeyCode::Numpad4 => Key::Numpad4,
            VirtualKeyCode::Numpad5 => Key::Numpad5,
            VirtualKeyCode::Numpad6 => Key::Numpad6,
            VirtualKeyCode::Numpad7 => Key::Numpad7,
            VirtualKeyCode::Numpad8 => Key::Numpad8,
            VirtualKeyCode::Numpad9 => Key::Numpad9,
            VirtualKeyCode::AbntC1 => Key::AbntC1,
            VirtualKeyCode::AbntC2 => Key::AbntC2,
            VirtualKeyCode::Add => Key::Add,
            VirtualKeyCode::Apostrophe => Key::Apostrophe,
            VirtualKeyCode::Apps => Key::Apps,
            VirtualKeyCode::At => Key::At,
            VirtualKeyCode::Ax => Key::Ax,
            VirtualKeyCode::Backslash => Key::Backslash,
            VirtualKeyCode::Calculator => Key::Calculator,
            VirtualKeyCode::Capital => Key::Capital,
            VirtualKeyCode::Colon => Key::Colon,
            VirtualKeyCode::Comma => Key::Comma,
            VirtualKeyCode::Convert => Key::Convert,
            VirtualKeyCode::Decimal => Key::Decimal,
            VirtualKeyCode::Divide => Key::Divide,
            VirtualKeyCode::Equals => Key::Equals,
            VirtualKeyCode::Grave => Key::Grave,
            VirtualKeyCode::Kana => Key::Kana,
            VirtualKeyCode::Kanji => Key::Kanji,
            VirtualKeyCode::LAlt => Key::LAlt,
            VirtualKeyCode::LBracket => Key::LBracket,
            VirtualKeyCode::LControl => Key::LControl,
            VirtualKeyCode::LShift => Key::LShift,
            VirtualKeyCode::LWin => Key::LWin,
            VirtualKeyCode::Mail => Key::Mail,
            VirtualKeyCode::MediaSelect => Key::MediaSelect,
            VirtualKeyCode::MediaStop => Key::MediaStop,
            VirtualKeyCode::Minus => Key::Minus,
            VirtualKeyCode::Multiply => Key::Multiply,
            VirtualKeyCode::Mute => Key::Mute,
            VirtualKeyCode::MyComputer => Key::MyComputer,
            VirtualKeyCode::NavigateForward => Key::NavigateForward,
            VirtualKeyCode::NavigateBackward => Key::NavigateBackward,
            VirtualKeyCode::NextTrack => Key::NextTrack,
            VirtualKeyCode::NoConvert => Key::NoConvert,
            VirtualKeyCode::NumpadComma => Key::NumpadComma,
            VirtualKeyCode::NumpadEnter => Key::NumpadEnter,
            VirtualKeyCode::NumpadEquals => Key::NumpadEquals,
            VirtualKeyCode::OEM102 => Key::OEM102,
            VirtualKeyCode::Period => Key::Period,
            VirtualKeyCode::PlayPause => Key::PlayPause,
            VirtualKeyCode::Power => Key::Power,
            VirtualKeyCode::PrevTrack => Key::PrevTrack,
            VirtualKeyCode::RAlt => Key::RAlt,
            VirtualKeyCode::RBracket => Key::RBracket,
            VirtualKeyCode::RControl => Key::RControl,
            VirtualKeyCode::RShift => Key::RShift,
            VirtualKeyCode::RWin => Key::RWin,
            VirtualKeyCode::Semicolon => Key::Semicolon,
            VirtualKeyCode::Slash => Key::Slash,
            VirtualKeyCode::Sleep => Key::Sleep,
            VirtualKeyCode::Stop => Key::Stop,
            VirtualKeyCode::Subtract => Key::Subtract,
            VirtualKeyCode::Sysrq => Key::Sysrq,
            VirtualKeyCode::Tab => Key::Tab,
            VirtualKeyCode::Underline => Key::Underline,
            VirtualKeyCode::Unlabeled => Key::Unlabeled,
            VirtualKeyCode::VolumeDown => Key::VolumeDown,
            VirtualKeyCode::VolumeUp => Key::VolumeUp,
            VirtualKeyCode::Wake => Key::Wake,
            VirtualKeyCode::WebBack => Key::WebBack,
            VirtualKeyCode::WebFavorites => Key::WebFavorites,
            VirtualKeyCode::WebForward => Key::WebForward,
            VirtualKeyCode::WebHome => Key::WebHome,
            VirtualKeyCode::WebRefresh => Key::WebRefresh,
            VirtualKeyCode::WebSearch => Key::WebSearch,
            VirtualKeyCode::WebStop => Key::WebStop,
            VirtualKeyCode::Yen => Key::Yen,
            VirtualKeyCode::Cut => Key::Cut,
            VirtualKeyCode::Copy => Key::Copy,
            VirtualKeyCode::Paste => Key::Paste,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
/// A unique ID for a gamepad that persists after the device is unplugged
pub struct GamepadId(
    #[cfg(feature = "gilrs")] pub(crate) gilrs::GamepadId,
    #[cfg(not(feature = "gilrs"))] usize,
);

impl PartialOrd for GamepadId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GamepadId {
    fn cmp(&self, other: &Self) -> Ordering {
        let a: usize = self.0.into();
        let b: usize = other.0.into();
        a.cmp(&b)
    }
}

#[derive(Debug)]
/// An event generated by a gamepad
pub enum GamepadEvent {
    /// The gamepad is available again
    Connected,
    /// The gamepad is no longer available
    Disconnected,
    /// A button has been pressed or released
    Button {
        button: GamepadButton,
        state: ElementState,
    },
    /// An axis has changed its value
    Axis { axis: GamepadAxis, value: f32 },
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
/// A button on a standard (d-pad, 2-stick, 4-button, 4-trigger) gamepad
pub enum GamepadButton {
    Start,
    Select,

    /// The north face button.
    ///
    /// * Nintendo: X
    /// * Playstation: Triangle
    /// * XBox: Y
    North,
    /// The south face button.
    ///
    /// * Nintendo: B
    /// * Playstation: X
    /// * XBox: A
    South,
    /// The east face button.
    ///
    /// * Nintendo: A
    /// * Playstation: Circle
    /// * XBox: B
    East,
    /// The west face button.
    ///
    /// * Nintendo: Y
    /// * Playstation: Square
    /// * XBox: X
    West,

    /// The left stick was pressed in as a button
    LeftStick,
    /// The right stick was pressed in as a button
    RightStick,

    LeftTrigger,
    RightTrigger,

    LeftShoulder,
    RightShoulder,

    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
/// The stick axes of a gamepad
pub enum GamepadAxis {
    LeftStickX,
    LeftStickY,

    RightStickX,
    RightStickY,
}
