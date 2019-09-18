use quick_lifecycle::traits::*;
use quick_lifecycle::{EventStream, Runner};

async fn my_app(mut events: EventStream) {
    while let Some(event) = events.next().await {
        println!("{:?}", event);
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let runner = Runner::new(event_loop);
    runner.run(my_app);
}
