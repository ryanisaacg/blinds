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
mod event;
mod event_stream;
mod run;
mod window;

pub use self::event::{Event, Modifiers, ElementState, Pointer, MouseButton, MouseScrollDelta, Key, GamepadId, GamepadEvent, GamepadButton, GamepadAxis};
pub use self::event_stream::EventStream;
pub use self::run::run;
pub use self::window::{CursorIcon, Settings, Window};

#[cfg(feature = "gl")]
pub use self::run::run_gl;

pub(crate) use self::event_stream::EventBuffer;
pub(crate) use self::window::WindowContents;
