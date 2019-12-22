use crate::Event;

use futures_util::future::poll_fn;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::sync::Arc;
use std::task::{Poll, Waker};

pub struct EventStream {
    buffer: Arc<RefCell<EventBuffer>>,
}

impl EventStream {
    pub fn new() -> EventStream {
        EventStream {
            buffer: Arc::new(RefCell::new(EventBuffer {
                events: VecDeque::new(),
                waker: None,
                ready: false,
            })),
        }
    }

    pub(crate) fn buffer(&self) -> Arc<RefCell<EventBuffer>> {
        self.buffer.clone()
    }

    pub fn next_event<'a>(&'a mut self) -> impl 'a + Future<Output = Option<Event>> {
        poll_fn(move |cx| {
            let mut buffer = self.buffer.borrow_mut();
            match buffer.events.pop_front() {
                Some(event) => Poll::Ready(Some(event)),
                None => {
                    if buffer.ready {
                        buffer.ready = false;
                        Poll::Ready(None)
                    } else {
                        buffer.waker = Some(cx.waker().clone());
                        Poll::Pending
                    }
                }
            }
        })
    }
}

pub(crate) struct EventBuffer {
    events: VecDeque<Event>,
    waker: Option<Waker>,
    ready: bool,
}

impl EventBuffer {
    pub fn push(&mut self, event: Event) {
        self.events.push_back(event);
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
        self.ready = true;
    }
}
