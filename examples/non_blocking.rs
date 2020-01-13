extern crate platter;
extern crate async_std;

use blinds::{run_custom, Event as BlindsEvent, EventContext, Settings, Window};
use platter::load_file;
use async_std::task::sleep;
use std::time::Duration;

#[derive(Debug)]
enum MyEvent {
    Blinds(BlindsEvent),
    AssetReady,
    Tick,
}

impl From<BlindsEvent> for MyEvent{
    fn from(e: BlindsEvent) -> Self { MyEvent::Blinds(e) }
}

fn main() {
    run_custom(Settings::default(), app);
}

async fn tick_loop(context: EventContext<MyEvent>) {
    // some sort of sleep would be nice
    loop {
        println!("Ticking");
        (&context).dispatch(MyEvent::Tick);
        sleep(Duration::from_secs(1)).await
    }
}

async fn app(_window: Window, mut context: EventContext<MyEvent>) {
    
    // Start a long-running "background" task
    context.spawn(tick_loop);

    // Start a task which completes
    context.spawn(|inner_context: EventContext<MyEvent>| async move {
        let _ = load_file("examples/non_blocking.rs").await;
        inner_context.dispatch(MyEvent::AssetReady)
    });

    loop {
        
        while let Some(ev) = context.stream.next_event().await {
            println!("Got event: {:?}", ev)
            // TODO: Kick off a new task every time you press a certain key (maybe only one pending at once)
        }

    }
}
