use winit::event::WindowEvent;

// TODO: redraw requested as separate event?
#[derive(Debug)]
pub enum Event {
    Close,
    Update,
    Input(WindowEvent),
    #[cfg(feature = "gamepad")]
    Gamepad(gilrs::Event),
}

