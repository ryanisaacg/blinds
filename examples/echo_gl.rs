use blinds::{run_gl, Event, EventStream, Key, Settings, Window};
use glow::Context;

fn main() {
    run_gl(Settings::default(), app);
}

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
