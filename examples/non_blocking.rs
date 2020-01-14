extern crate async_std;
extern crate platter;

use async_std::task::sleep;
use blinds::{run_custom, Event as BlindsEvent, EventContext, Key, Settings, Window};
use platter::load_file;
use std::io::Error as IOError;
use std::time::Duration;

#[derive(Debug)]
enum MyEvent {
    Blinds(BlindsEvent),
    AssetReady(Result<usize, IOError>),
    Ticked,
    Handled,
}

impl From<BlindsEvent> for MyEvent {
    fn from(e: BlindsEvent) -> Self {
        MyEvent::Blinds(e)
    }
}

fn main() {
    run_custom(Settings::default(), app);
}

async fn tick_loop(context: EventContext<MyEvent>) {
    // some sort of sleep would be nice
    loop {
        println!("Ticking");
        (&context).dispatch(MyEvent::Ticked);
        sleep(Duration::from_secs(1)).await
    }
}

async fn button_handler(context: EventContext<MyEvent>) {
    println!("Handling!");
    (&context).dispatch(MyEvent::Handled);
}

async fn app(_window: Window, mut context: EventContext<MyEvent>) {
    // Start a long-running "background" task
    context.spawn(tick_loop);

    // Start a third-party async task, and notify a first-party event on completion
    context.spawn(|inner_context: EventContext<MyEvent>| {
        async move {
            let result = load_file("examples/non_blocking.rs").await;
            let content_length = result.map(|vec| vec.len());
            inner_context.dispatch(MyEvent::AssetReady(content_length))
        }
    });

    'outer: loop {
        while let Some(ev) = context.stream().next_event().await {
            println!("Got event: {:?}", ev);

            if let MyEvent::Blinds(BlindsEvent::KeyboardInput {
                key: Key::Escape, ..
            }) = ev
            {
                // Exit the main loop, eventually causing the window to close and them program to terminate
                break 'outer;
            }

            if let MyEvent::Blinds(BlindsEvent::KeyboardInput {
                key: Key::Space, ..
            }) = ev
            {
                // Start a first-party
                context.spawn(button_handler);
            }
        }
    }
}
