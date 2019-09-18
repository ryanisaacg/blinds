use crate::{Event, EventStream};
use futures_util::task::LocalSpawnExt;
use futures_executor::LocalPool;
use std::future::Future;
use winit::event::Event as WinitEvent;
use winit::event_loop::EventLoop;

// TODO: add gilrs events
// TODO: add timing handling

pub struct Runner {
    event_loop: EventLoop<()>
}

impl Runner {
    pub fn new(event_loop: EventLoop<()>) -> Runner {
        Runner {
            event_loop
        }
    }

    pub fn run<F, T>(self, app: F) -> !
            where T: 'static + Future<Output = ()>, F: FnOnce(EventStream) -> T {
        let stream = EventStream::new();
        let buffer = stream.buffer();


        let mut pool = LocalPool::new();
        let mut spawner = pool.spawner();
        spawner.spawn_local(app(stream)).expect("Failed to start application");

        self.event_loop.run(move |event, _, _| {
            match event {
                WinitEvent::WindowEvent { event, .. } => {
                    buffer.borrow_mut().push(Event::Input(event));
                }
                WinitEvent::LoopDestroyed => {
                    buffer.borrow_mut().push(Event::Close);
                    pool.run_until_stalled();
                }
                WinitEvent::EventsCleared => {
                    buffer.borrow_mut().push(Event::Update);
                    pool.run_until_stalled();
                }
                _ => ()
            }
        })
    }
}
