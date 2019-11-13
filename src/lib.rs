mod event;
mod event_stream;
mod run;
mod window;

// TODO: add gilrs events
// TODO: add timing handling
// TODO: provide custom windowbuilder
// TODO: close requests or just let the future finish
// TODO: give the graphics context access to the windowedcontext objects

pub use self::event::*;
pub use self::event_stream::EventStream;
pub use self::run::run;
pub use self::window::{CursorIcon, Window, Settings};

#[cfg(feature = "gl")]
pub use self::run::run_gl;

pub(crate) use self::event_stream::EventBuffer;

pub mod traits {
    pub use futures_util::stream::StreamExt;
}
