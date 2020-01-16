use blinds::{run, Event, EventStream, Key, Settings, Window};

fn main() {
    run(Settings::default(), app);
}

async fn app(_window: Window, mut events: EventStream) {
    'outer: loop {
        while let Some(ev) = events.next_event().await {
            if let Event::KeyboardInput(Key::Escape, _) = ev {
                break 'outer;
            }
            println!("{:?}", ev);
        }
    }
}
