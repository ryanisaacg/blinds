extern crate platter;

use blinds::{run, Event, EventStream, Key, Settings, Window};

use platter::load_file;
use std::future::Future;
use std::string::String;

fn main() {
    run(Settings::default(), app);
}

#[cfg(target_arch = "wasm32")]
async fn echo_request() -> Result<String> {
    let url = "https://postman-echo.com/get?foo1=bar1";
    let response = load_file(url).await?;
    Ok(format!("Response payload: {} bytes", response.len()))
}

async fn app(_window: Window, mut events: EventStream) {
    
    // Kick off an inialization task, and block on its result
    
    // Setup a custom CustomEventStream<CustomEvent>
    
    // Kick off a loading task, and don't block

    loop {
        
        while let Some(ev) = events.next_event().await {
            println!("Got event: {:?}", ev)
            // Kick off a new task every time you press a certain key (maybe only one pending at once)
        }

        // while let Some(ev) = custom_events.next_event().await {
        //     println!("Got custom event: {:?}", ev)
        // }
        
    }
}
