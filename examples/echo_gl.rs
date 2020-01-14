#[cfg(feature = "gl")]
use blinds::{run_gl, Event, EventStream, Key, Settings, Window};
#[cfg(feature = "gl")]
use glow::Context;

fn main() {
    #[cfg(feature = "gl")]
    run_gl(Settings::default(), app);
}

#[cfg(feature = "gl")]
async fn app(_window: Window, _ctx: Context, mut events: EventStream<Event>) {
    'outer: loop {
        while let Some(ev) = events.next_event().await {
            if let Event::KeyboardInput {
                key: Key::Escape, ..
            } = ev
            {
                break 'outer;
            }
            println!("{:?}", ev);
        }
        // TODO: use the glow context for something basic
    }
}
