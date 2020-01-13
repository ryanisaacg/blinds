extern crate platter;

use blinds::{run_custom, Event as BlindsEvent, EventContext, Settings, Window};
use platter::load_file;

#[derive(Debug)]
enum MyEvent {
    Blinds(BlindsEvent),
    AssetReady,
}

impl From<BlindsEvent> for MyEvent{
    fn from(e: BlindsEvent) -> Self { MyEvent::Blinds(e) }
}

fn main() {
    run_custom(Settings::default(), app);
}

async fn load(context: EventContext<MyEvent>) {
    let f = load_file("examples/non_blocking.rs").await;
    println!("File ready: {:?}", f);
    context.dispatch(MyEvent::AssetReady);
}

async fn app(_window: Window, mut context: EventContext<MyEvent>) {
    
    // Kick off a loading task, and don't block
    context.spawn(load);

    // TODO: I would really like to use a closure here. Uncomment, it fails

    // context.clone().spawn(|inner_context: EventContext<MyEvent>| async {
    //     let _ = load_file("examples/non_blocking.rs").await;
    //     inner_context.clone().dispatch(MyEvent::AssetReady);
    //     ()
    // });

    loop {
        
        println!("Entering loop iteration");

        while let Some(ev) = context.stream.next_event().await {
            println!("Got event: {:?}", ev)
            // Kick off a new task every time you press a certain key (maybe only one pending at once)
        }

        println!("Ending loop iteration");

    }
}
