use crate::Event;
use futures_core::stream::Stream;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

pub struct EventStream {
    buffer: Arc<RefCell<EventBuffer>>,
}

impl EventStream {
    pub fn new() -> EventStream {
        EventStream {
            buffer: Arc::new(RefCell::new(EventBuffer {
                events: VecDeque::new(),
                waker: None,
            })),
        }
    }

    pub(crate) fn buffer(&self) -> Arc<RefCell<EventBuffer>> {
        self.buffer.clone()
    }
}

impl Stream for EventStream {
    type Item = Event;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let mut buffer = self.buffer.borrow_mut();

        match buffer.events.pop_front() {
            Some(event) => Poll::Ready(Some(event)),
            None => {
                buffer.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

pub(crate) struct EventBuffer {
    events: VecDeque<Event>,
    waker: Option<Waker>,
}

impl EventBuffer {
    pub fn push(&mut self, event: Event) {
        self.events.push_back(event);
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
}
