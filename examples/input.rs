use quick_lifecycle::traits::*;
use quick_lifecycle::{Keyboard, Mouse, EventStream, Runner};

async fn my_app(mut events: EventStream) {
    let mut keyboard = Keyboard::new();
    let mut mouse = Mouse::new();

    while let Some(event) = events.next().await {
        keyboard.process_event(&event);
        mouse.process_event(&event);

        println!("{:?}", event);
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let runner = Runner::new(event_loop);
    runner.run(my_app);
}
