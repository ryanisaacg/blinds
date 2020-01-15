use blinds::{run, EventStream, Settings, Window};
use futures_util::future::{Either, join_all, ready, select};

fn main() {
    run(Settings::default(), app);
}

async fn app(_window: Window, mut events: EventStream) {
    let mut resource_loading = Some(join_all(vec![
        ready("Resource A!"),
        ready("Resource B"),
        ready("Resource C"),
        ready("Resource D")
    ]));
    let mut loaded_resource: Option<Vec<&'static str>> = None;
    loop {
        match loaded_resource.as_mut() {
            Some(_resource) => {
                while let Some(ev) = events.next_event().await {
                    println!("Resources loaded, event: {:?}", ev);
                }
            },
            None => {
                let loading = resource_loading.take().expect("resource neither loaded or loading");
                match select(loading, events.next_event()).await {
                    Either::Left((loaded, event_fut)) => {
                        loaded_resource = Some(loaded);
                        println!("Loaded resources");
                        let event = event_fut.await;
                        println!("Resources loaded, event: {:?}", event);
                    }
                    Either::Right((event, resource_fut)) => {
                        resource_loading = Some(resource_fut);
                        println!("Resources loading, event: {:?}", event);
                    }
                }
            }
        }
    }
}



