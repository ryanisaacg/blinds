mod event;
mod event_stream;
mod run;
mod window;

pub use self::event::*;
pub use self::event_stream::EventStream;
pub use self::run::run;
pub use self::window::{CursorIcon, Settings, Window};

#[cfg(feature = "gl")]
pub use self::run::run_gl;

pub(crate) use self::event_stream::EventBuffer;
pub(crate) use self::window::WindowContents;

pub mod traits {
    pub use futures_util::stream::StreamExt;
}
