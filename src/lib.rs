mod event;
mod event_stream;
mod run;
mod window;

// TODO: respect window icons
// TODO: error handling

pub use self::event::*;
pub use self::event_stream::EventStream;
pub use self::run::run;
pub use self::window::{CursorIcon, Window, Settings};

#[cfg(feature = "gl")]
pub use self::run::run_gl;

pub(crate) use self::event_stream::EventBuffer;
pub(crate) use self::window::WindowContents;

pub mod traits {
    pub use futures_util::stream::StreamExt;
}
