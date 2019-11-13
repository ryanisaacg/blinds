use blinds::traits::*;
use blinds::{Event, EventStream, Key, Settings, Window, run};


fn main() {
    run(Settings::default(), app);
}

async fn app(_window: Window, mut events: EventStream) {
    while let Some(frame) = events.next().await {
        let mut close = false;
        for event in frame {
            if let Event::KeyboardInput { key: Key::Escape, .. } = event {
                close = true;
            }
            println!("{:?}", event);
        }
        if close {
            break;
        }
    }
}
