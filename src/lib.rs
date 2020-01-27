//! `blinds` covers up the details of your windowing for you, by providing an async API.
//!
//! A quick example of some code that prints all incoming events:
//! ```no_run
//! use blinds::{run, Event, EventStream, Key, Settings, Window};
//!
//! run(Settings::default(), app);
//!
//! async fn app(_window: Window, mut events: EventStream) {
//!     loop {
//!         while let Some(ev) = events.next_event().await {
//!             println!("{:?}", ev);
//!         }
//!     }
//! }
//! ```
//!
//! The core of blinds is [`run`], which executes your app and provides your [`Window`] and
//! [`EventStream`] instances.
//!
//! [`run`]: run()
//! [`Window`]: Window
//! [`EventStream`]: EventStream
mod event_stream;
mod run;
mod settings;
mod window;

pub mod event;

pub use self::event::{Event, GamepadAxis, GamepadButton, GamepadId, Key, MouseButton, PointerId};
pub use self::event_stream::EventStream;
pub use self::run::run;
pub use self::settings::{CursorIcon, Settings};
pub use self::window::Window;

#[cfg(feature = "gl")]
pub use self::run::run_gl;

pub(crate) use self::event_stream::EventBuffer;
pub(crate) use self::window::WindowContents;
