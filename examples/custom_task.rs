
use blinds::{run, EventStream, Event as BlindsEvent, Key, Settings, Window, ElementState};
use futures_util::task::LocalSpawnExt;
use futures_core::stream::Stream;
use futures_util::stream::FuturesUnordered;
use std::sync::Arc;
use std::cell::RefCell;
use std::pin::Pin;
use futures_util::future::poll_fn;
use std::future::Future;
use std::task::{Poll, Waker};
use futures_util::future::LocalFutureObj;
use log::{debug, info, trace};

fn set_logger() {
    #[cfg(target_arch = "wasm32")]
    web_logger::custom_init(web_logger::Config {
        level: log::Level::Debug,
    });
    #[cfg(not(target_arch = "wasm32"))]
    simple_logger::init_with_level(log::Level::Debug).expect("A logger was already initialized");
}

fn main() {
    set_logger();
    run(Settings::default(), app);
}

#[derive(Debug)]
enum CustomEvent {
    OnePingOnly,
    Ticked,
}

#[cfg(not(target_arch = "wasm32"))]
mod sleep {
    use async_std::task::sleep;
    use std::time::Duration;
    
    pub async fn sleep_1() { sleep(Duration::from_secs(1)).await }
}

#[cfg(all(feature = "stdweb", target_arch = "wasm32"))]
mod sleep {
    extern crate std_web;

    use std_web::web::wait;

    pub async fn sleep_1() { wait(1000).await }
}

async fn tick_loop<'a>(task_context: MyTaskContext<'a, CustomEvent>) {
    loop {
        task_context.dispatch(CustomEvent::Ticked);
        sleep::sleep_1().await
    }
}

struct MyTaskContext<'a, E> {
    events: Arc<RefCell<Vec<E>>>,
    futures: Arc<RefCell<FuturesUnordered<LocalFutureObj<'a, ()>>>>,
    task_waker: Arc<RefCell<Option<Waker>>>,
}

impl <'a, E> Clone for MyTaskContext<'a, E> {
    fn clone(&self) -> Self {
        MyTaskContext {
            events: self.events.clone(),
            futures: self.futures.clone(),
            task_waker: self.task_waker.clone(),
        }
    }
}

impl<'a, E> MyTaskContext<'a, E> {
    fn new() -> Self {
        MyTaskContext {
            events: Arc::new(RefCell::new(Vec::new())),
            futures: Arc::new(RefCell::new(FuturesUnordered::new())),
            task_waker: Arc::new(RefCell::new(None)),
        }
    }

    async fn run_until_stalled(&mut self) {
        poll_fn(move |cx| {
            let mut x = self.futures.borrow_mut();
            loop {
                let pinned_pool = Pin::new(&mut *x);
                let pool_state = pinned_pool.poll_next(cx);
                trace!("Task context run pool_state: {:?}", pool_state);
                match pool_state {
                    Poll::Pending => break Poll::Ready(()),
                    Poll::Ready(Some(_)) => {
                        debug!("Task finished");
                        continue
                    }
                    Poll::Ready(None) => {
                        self.task_waker.replace(Some(cx.waker().clone()));
                        break Poll::Ready(())
                    }
                }
            }
        }).await
    }

    fn spawn<Fut>(&mut self, task: Fut) where Fut: 'static + Future<Output = ()> {
        debug!("Spawning new task");
        self.futures.borrow().spawn_local(task).expect("");
        if let Some(waker) = self.task_waker.replace(None) {
            waker.wake();
        }
    }

    fn dispatch(&self, event: E) {
        self.events.borrow_mut().push(event)
    }

    fn drain(&self) -> Vec<E> {
        self.events.replace(Vec::new())
    }
}

async fn app(_window: Window, mut event_stream: EventStream) {
    let mut task_context: MyTaskContext<CustomEvent> = MyTaskContext::new();

    task_context.spawn(tick_loop(task_context.clone()));

    let cloned_task_context = task_context.clone();
    task_context.spawn(async move {
        cloned_task_context.dispatch(CustomEvent::OnePingOnly);
        sleep::sleep_1().await
    });

    'main: loop {
        // debug!("Main loop wrapped around");
        task_context.run_until_stalled().await;

        for custom_event in task_context.drain().into_iter() {
            info!("CustomEvent: {:?}", custom_event)
        }

        while let Some(ev) = event_stream.next_event().await {
            if let BlindsEvent::KeyboardInput {
                key: Key::Escape, ..
            } = ev
            {
                break 'main;
            }

            if let BlindsEvent::KeyboardInput {
                key: Key::P, state: ElementState::Pressed
            } = ev
            {
                let cloned_task_context = task_context.clone();
                task_context.spawn(async move {
                    cloned_task_context.dispatch(CustomEvent::OnePingOnly);
                    sleep::sleep_1().await
                });
            }

            info!("BlindsEvent: {:?}", ev);
        }
        
    }
}
