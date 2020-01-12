extern crate platter;

use blinds::{run_custom, Event as BlindsEvent, EventContext, Settings, Window};

#[derive(Debug)]
enum MyEvent {
    Blinds(BlindsEvent)
}

impl From<BlindsEvent> for MyEvent{
    fn from(e: BlindsEvent) -> Self { MyEvent::Blinds(e) }
}

fn main() {
    run_custom(Settings::default(), app);
}

async fn app(_window: Window, mut events: EventContext<MyEvent>) {
    
    // Kick off an inialization task, and block on its result
    
    // Setup a custom CustomEventStream<CustomEvent>
    
    // Kick off a loading task, and don't block

    loop {
        
        while let Some(ev) = events.stream.next_event().await {
            println!("Got event: {:?}", ev)
            // Kick off a new task every time you press a certain key (maybe only one pending at once)
        }

        // while let Some(ev) = custom_events.next_event().await {
        //     println!("Got custom event: {:?}", ev)
        // }
        
    }
}
