// Create a custom event stream built on top of EventStream
// Use the select function to accomplish this

use blinds::{run, EventStream, Settings, Window};
use futures_util::future::{Either, FutureExt, join_all, ready, select};
use std::future::Future;

fn main() {
    run(Settings::default(), app);
}

async fn app(_window: Window, events: EventStream) {
    let mut custom_stream = CustomStream::new(events, join_all(vec![
        ready("Resource A!"),
        ready("Resource B"),
        ready("Resource C"),
        ready("Resource D")
    ]));
    loop {
        match custom_stream.next_event().await {
            CustomEvent::Event(Some(ev)) => {
                println!("Blinds event: {:?}", ev);
            }
            CustomEvent::Event(None) => {
                // do nothing
            }
            CustomEvent::Loaded(_resource) => {
                println!("Resource loaded");
            }
        }
    }
}

type Resource = Vec<&'static str>;

struct CustomStream {
    events: EventStream,
    resource_task: Option<Box<dyn Unpin + Future<Output = Resource>>>,
}

enum CustomEvent {
    Event(Option<blinds::Event>),
    Loaded(Resource),
}

impl CustomStream {
    fn new(events: EventStream, fut: impl 'static + Unpin + Future<Output = Resource>) -> CustomStream {
        CustomStream {
            events,
            resource_task: Some(Box::new(fut)),
        }
    }

    async fn next_event(&mut self) -> CustomEvent {
        match self.resource_task.take() {
            Some(task) => {
                let ev = self.events.next_event().map(|ev| CustomEvent::Event(ev));
                match select(task, ev).await {
                    Either::Left((resource, _)) => CustomEvent::Loaded(resource),
                    Either::Right((event, resource_task)) => {
                        self.resource_task = Some(resource_task);
                        event
                    }
                }
            }
            None => CustomEvent::Event(self.events.next_event().await),
        }
    }
}
