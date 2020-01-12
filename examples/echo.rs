use blinds::{run, Event, EventStream, Key, Settings, Window};

fn main() {
    run(Settings::default(), app);
}

async fn app(_window: Window, mut events: EventStream<Event>) {
    loop {
        while let Some(ev) = events.next_event().await {
            if let Event::KeyboardInput {
                key: Key::Escape, ..
            } = ev
            {
                break;
            }
            println!("{:?}", ev);
        }
    }
}
