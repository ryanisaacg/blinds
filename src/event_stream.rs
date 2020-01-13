
use futures_util::future::poll_fn;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::sync::Arc;
use std::task::{Poll, Waker};

/// The source of events for a `blinds` application
///
/// An `EventStream` instance is supplied by [`run`], so creating one is not necessary. Use the
/// [`next_event`] function to wait for [`Event`]s.
///
/// [`next_event`]: EventStream::next_event
/// [`Event`]: Event
/// [`run`]: crate::run()
pub struct EventStream<E> {
    buffer: Arc<RefCell<EventBuffer<E>>>,
}

// All of these custom Clone are smelly
impl<E> Clone for EventStream<E> {
    fn clone(&self) -> Self {
        EventStream { buffer: self.buffer() }
    }
}

impl<E> EventStream<E> {
    pub(crate) fn new() -> Self {
        EventStream {
            buffer: Arc::new(RefCell::new(EventBuffer {
                events: VecDeque::new(),
                waker: None,
                ready: false,
            })),
        }
    }

    pub(crate) fn buffer(&self) -> Arc<RefCell<EventBuffer<E>>> {
        self.buffer.clone()
    }

    /// Returns a future that will provide the next [`E`], or None if the events are exhausted
    ///
    /// If there are no events, the Future will wait until new events are received, allowing the OS
    /// or browser to take back control of the event loop. If this doesn't get run, desktop windows
    /// will freeze and browser windows will lock up, so it's important to call and `.await` the
    /// Future even if the events are ignored.
    ///
    /// [`E`]: Event
    pub fn next_event<'a>(&'a mut self) -> impl 'a + Future<Output = Option<E>> {
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

pub(crate) struct EventBuffer<E> {
    events: VecDeque<E>,
    waker: Option<Waker>,
    ready: bool,
}

impl <E> EventBuffer<E> {
    pub fn push(&mut self, event: E) {
        self.events.push_back(event);
        self.mark_ready();
    }

    pub fn mark_ready(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
        self.ready = true;
    }
}
